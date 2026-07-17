#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    MerkleRoot,
    UserClaim(Address),
}

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {
    /// Initialize the contract with an admin, a token to distribute, and the merkle root.
    pub fn init(env: Env, admin: Address, token: Address, merkle_root: BytesN<32>) {
        admin.require_auth();
        
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::MerkleRoot, &merkle_root);
    }

    /// Update the Merkle root. Only the admin can do this.
    pub fn update_root(env: Env, new_root: BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::MerkleRoot, &new_root);
    }
}
