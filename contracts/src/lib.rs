#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token,
    xdr::ToXdr, Address, Bytes, BytesN, Env, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AirdropError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    AlreadyClaimed = 4,
    InvalidProof = 5,
    ZeroAmount = 6,
}

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
    /// Initialize the contract with an admin, a token to distribute, and the Merkle root.
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        merkle_root: BytesN<32>,
    ) -> Result<(), AirdropError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            return Err(AirdropError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::MerkleRoot, &merkle_root);

        env.events().publish((symbol_short!("init"), admin), merkle_root);
        Ok(())
    }

    /// Update the Merkle root. Only admin can call this.
    pub fn update_root(env: Env, new_root: BytesN<32>) -> Result<(), AirdropError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(AirdropError::NotInitialized)?;
        admin.require_auth();

        env.storage().instance().set(&DataKey::MerkleRoot, &new_root);
        env.events().publish((symbol_short!("upd_root"), admin), new_root);
        Ok(())
    }

    /// Compute the Merkle leaf hash for a given claimant and amount.
    /// Leaf = SHA-256( claimant.to_xdr() || amount.to_be_bytes() )
    pub fn compute_leaf(env: Env, claimant: Address, amount: i128) -> BytesN<32> {
        let mut data = Bytes::new(&env);
        data.append(&claimant.to_xdr(&env));
        let be = amount.to_be_bytes();
        data.append(&Bytes::from_slice(&env, &be));
        env.crypto().sha256(&data).into()
    }

    /// Verify a Merkle proof for a given leaf against the stored Merkle root.
    pub fn verify_proof(
        env: Env,
        claimant: Address,
        amount: i128,
        proof: Vec<BytesN<32>>,
    ) -> Result<bool, AirdropError> {
        let root: BytesN<32> = env
            .storage()
            .instance()
            .get(&DataKey::MerkleRoot)
            .ok_or(AirdropError::NotInitialized)?;

        let leaf = Self::compute_leaf(env.clone(), claimant, amount);
        Ok(Self::verify_merkle_proof(&env, leaf, proof, root))
    }

    /// Claim airdrop tokens using a valid Merkle proof.
    pub fn claim(
        env: Env,
        claimant: Address,
        amount: i128,
        proof: Vec<BytesN<32>>,
    ) -> Result<(), AirdropError> {
        claimant.require_auth();

        if amount <= 0 {
            return Err(AirdropError::ZeroAmount);
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(AirdropError::NotInitialized)?;

        let root: BytesN<32> = env
            .storage()
            .instance()
            .get(&DataKey::MerkleRoot)
            .ok_or(AirdropError::NotInitialized)?;

        // Check duplicate claim
        if Self::is_claimed(env.clone(), claimant.clone()) {
            return Err(AirdropError::AlreadyClaimed);
        }

        // Verify Merkle proof
        let leaf = Self::compute_leaf(env.clone(), claimant.clone(), amount);
        if !Self::verify_merkle_proof(&env, leaf, proof, root) {
            return Err(AirdropError::InvalidProof);
        }

        // Mark as claimed in persistent storage
        env.storage()
            .persistent()
            .set(&DataKey::UserClaim(claimant.clone()), &true);

        // Transfer tokens to claimant
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &claimant, &amount);

        // Emit claim event
        env.events().publish((symbol_short!("claim"), claimant), amount);

        Ok(())
    }

    /// Admin emergency withdrawal of remaining tokens.
    pub fn withdraw_remaining(env: Env, to: Address, amount: i128) -> Result<(), AirdropError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(AirdropError::NotInitialized)?;
        admin.require_auth();

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(AirdropError::NotInitialized)?;

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        env.events().publish((symbol_short!("withdraw"), to), amount);
        Ok(())
    }

    /// Check if an address has already claimed their tokens.
    pub fn is_claimed(env: Env, claimant: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::UserClaim(claimant))
            .unwrap_or(false)
    }

    /// Read the current Merkle root.
    pub fn get_merkle_root(env: Env) -> Result<BytesN<32>, AirdropError> {
        env.storage()
            .instance()
            .get(&DataKey::MerkleRoot)
            .ok_or(AirdropError::NotInitialized)
    }

    /// Read the token contract address.
    pub fn get_token(env: Env) -> Result<Address, AirdropError> {
        env.storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(AirdropError::NotInitialized)
    }

    /// Read the admin address.
    pub fn get_admin(env: Env) -> Result<Address, AirdropError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(AirdropError::NotInitialized)
    }

    /// Internal Merkle proof verification helper (sorted pair hashing).
    fn verify_merkle_proof(
        env: &Env,
        leaf: BytesN<32>,
        proof: Vec<BytesN<32>>,
        root: BytesN<32>,
    ) -> bool {
        let mut current = leaf;

        for p in proof.iter() {
            let mut combined = Bytes::new(env);
            if current <= p {
                combined.append(&Bytes::from(current));
                combined.append(&Bytes::from(p));
            } else {
                combined.append(&Bytes::from(p));
                combined.append(&Bytes::from(current));
            }
            current = env.crypto().sha256(&combined).into();
        }

        current == root
    }
}

#[cfg(test)]
mod test;
