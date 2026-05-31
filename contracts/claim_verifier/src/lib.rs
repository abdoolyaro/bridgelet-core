#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, panic_with_error, Address, BytesN, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Unauthorized = 1,
}

#[contract]
pub struct ClaimVerifierContract;

#[contractimpl]
impl ClaimVerifierContract {
    pub fn verify(env: Env, _destination: Address, _nonce: u64, signature: BytesN<64>) {
        let valid_signature = BytesN::from_array(&env, &[0u8; 64]);
        if signature != valid_signature {
            panic_with_error!(env, Error::Unauthorized);
        }
    }
}
