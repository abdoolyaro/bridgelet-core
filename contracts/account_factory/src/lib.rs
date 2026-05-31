#![no_std]

mod errors;
mod events;
mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env};

pub use errors::Error;
pub use events::AccountDeployed;

#[contract]
pub struct AccountFactoryContract;

#[contractimpl]
impl AccountFactoryContract {
    // Implementation coming in Issues #54 and #55
}
