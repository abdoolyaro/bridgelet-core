use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    EphemeralWasmHash,
    DeployedAccounts,
    AllAccounts,
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_ephemeral_wasm_hash(env: &Env, hash: &soroban_sdk::BytesN<32>) {
    env.storage()
        .instance()
        .set(&DataKey::EphemeralWasmHash, hash);
}

pub fn get_ephemeral_wasm_hash(env: &Env) -> Option<soroban_sdk::BytesN<32>> {
    env.storage().instance().get(&DataKey::EphemeralWasmHash)
}
