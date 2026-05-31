#![no_std]

mod errors;
mod events;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

pub use errors::Error;
pub use events::VerificationSucceeded;

#[contract]
pub struct ClaimVerifierContract;

#[contractimpl]
impl ClaimVerifierContract {
    /// Initialize with the authorized signer public key
    ///
    /// # Arguments
    /// * `authorized_signer` - Ed25519 public key (32 bytes)
    ///
    /// # Errors
    /// * `Error::AlreadyInitialized` - called more than once
    pub fn initialize(env: Env, authorized_signer: BytesN<32>) -> Result<(), Error> {
        if env.storage().instance().has(&"signer") {
            return Err(Error::AlreadyInitialized);
        }

        env.storage()
            .instance()
            .set(&"signer", &authorized_signer);

        Ok(())
    }

    /// Verify an Ed25519 sweep authorization signature
    ///
    /// Message format matches sweep_controller/authorization.rs:
    /// hash(destination + nonce + contract_id)
    ///
    /// # Arguments
    /// * `destination` - Destination wallet address
    /// * `nonce` - Current sweep nonce
    /// * `signature` - Ed25519 signature (64 bytes)
    ///
    /// # Errors
    /// * `Error::NotInitialized` - contract not initialized
    /// * `Error::AuthorizedSignerNotSet` - no signer stored
    /// * `Error::SignatureVerificationFailed` - signature is invalid
    pub fn verify(
        env: Env,
        destination: Address,
        nonce: u64,
        signature: BytesN<64>,
    ) -> Result<(), Error> {
        // Get authorized signer
        let authorized_signer: BytesN<32> = env
            .storage()
            .instance()
            .get(&"signer")
            .ok_or(Error::AuthorizedSignerNotSet)?;

        // Construct message: hash(destination + nonce + contract_id)
        let message = Self::construct_message(&env, &destination, nonce);

        // Verify Ed25519 signature
        env.crypto()
            .ed25519_verify(&authorized_signer, &message.into(), &signature);

        // Emit success event
        events::emit_verification_succeeded(&env, destination, nonce);

        Ok(())
    }

    // Private helper — constructs the message hash identical to
    // sweep_controller/authorization.rs construct_sweep_message
    fn construct_message(env: &Env, destination: &Address, nonce: u64) -> BytesN<32> {
        use soroban_sdk::xdr::ToXdr;

        let contract_id = env.current_contract_address();
        let mut message = soroban_sdk::Bytes::new(env);

        let dest_bytes = destination.to_xdr(env);
        message.append(&dest_bytes);

        message.push_back(((nonce >> 56) & 0xFF) as u8);
        message.push_back(((nonce >> 48) & 0xFF) as u8);
        message.push_back(((nonce >> 40) & 0xFF) as u8);
        message.push_back(((nonce >> 32) & 0xFF) as u8);
        message.push_back(((nonce >> 24) & 0xFF) as u8);
        message.push_back(((nonce >> 16) & 0xFF) as u8);
        message.push_back(((nonce >> 8) & 0xFF) as u8);
        message.push_back((nonce & 0xFF) as u8);

        let contract_bytes = contract_id.to_xdr(env);
        message.append(&contract_bytes);

        env.crypto().sha256(&message).into()
    }
}

