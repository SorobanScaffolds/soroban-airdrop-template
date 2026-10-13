#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, Bytes, BytesN, Env, Vec,
};

fn hash_pair(env: &Env, a: &BytesN<32>, b: &BytesN<32>) -> BytesN<32> {
    let mut combined = Bytes::new(env);
    if a <= b {
        combined.append(&Bytes::from(a.clone()));
        combined.append(&Bytes::from(b.clone()));
    } else {
        combined.append(&Bytes::from(b.clone()));
        combined.append(&Bytes::from(a.clone()));
    }
    env.crypto().sha256(&combined).into()
}

struct TestSetup<'a> {
    env: Env,
    admin: Address,
    token_id: Address,
    token_admin_client: StellarAssetClient<'a>,
    token_client: TokenClient<'a>,
    contract_id: Address,
    contract_client: AirdropContractClient<'a>,
}

impl<'a> TestSetup<'a> {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_admin_client = StellarAssetClient::new(&env, &token_id.address());
        let token_client = TokenClient::new(&env, &token_id.address());

        let contract_id = env.register(AirdropContract, ());
        let contract_client = AirdropContractClient::new(&env, &contract_id);

        TestSetup {
            env,
            admin,
            token_id: token_id.address(),
            token_admin_client,
            token_client,
            contract_id,
            contract_client,
        }
    }
}

#[test]
fn test_initialization() {
    let setup = TestSetup::new();
    let root = BytesN::from_array(&setup.env, &[42u8; 32]);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);

    assert_eq!(setup.contract_client.get_admin(), setup.admin);
    assert_eq!(setup.contract_client.get_token(), setup.token_id);
    assert_eq!(setup.contract_client.get_merkle_root(), root);

    // Initializing again should fail
    let res = setup
        .contract_client
        .try_init(&setup.admin, &setup.token_id, &root);
    assert_eq!(res, Err(Ok(AirdropError::AlreadyInitialized)));
}

#[test]
fn test_update_root() {
    let setup = TestSetup::new();
    let root1 = BytesN::from_array(&setup.env, &[1u8; 32]);
    let root2 = BytesN::from_array(&setup.env, &[2u8; 32]);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root1);
    assert_eq!(setup.contract_client.get_merkle_root(), root1);

    setup.contract_client.update_root(&root2);
    assert_eq!(setup.contract_client.get_merkle_root(), root2);
}

#[test]
fn test_successful_claim_and_double_claim_prevention() {
    let setup = TestSetup::new();

    let user1 = Address::generate(&setup.env);
    let user2 = Address::generate(&setup.env);
    let amount1: i128 = 500_000_000; // 50 tokens
    let amount2: i128 = 300_000_000; // 30 tokens

    // Calculate leaves
    let leaf1 = AirdropContract::compute_leaf(setup.env.clone(), user1.clone(), amount1);
    let leaf2 = AirdropContract::compute_leaf(setup.env.clone(), user2.clone(), amount2);

    // Merkle root of 2 items is hash of sorted pair
    let root = hash_pair(&setup.env, &leaf1, &leaf2);

    // Initialize contract
    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);

    // Mint tokens to the airdrop contract
    setup
        .token_admin_client
        .mint(&setup.contract_id, &1_000_000_000);
    assert_eq!(setup.token_client.balance(&setup.contract_id), 1_000_000_000);

    // Proof for user1 is [leaf2]
    let mut proof1 = Vec::new(&setup.env);
    proof1.push_back(leaf2.clone());

    // Proof for user2 is [leaf1]
    let mut proof2 = Vec::new(&setup.env);
    proof2.push_back(leaf1.clone());

    // Verify proof function
    assert!(setup
        .contract_client
        .verify_proof(&user1, &amount1, &proof1));
    assert!(setup
        .contract_client
        .verify_proof(&user2, &amount2, &proof2));

    // Initially not claimed
    assert!(!setup.contract_client.is_claimed(&user1));
    assert!(!setup.contract_client.is_claimed(&user2));

    // User 1 claims
    setup.contract_client.claim(&user1, &amount1, &proof1);

    // Verify state after claim
    assert!(setup.contract_client.is_claimed(&user1));
    assert_eq!(setup.token_client.balance(&user1), amount1);
    assert_eq!(
        setup.token_client.balance(&setup.contract_id),
        1_000_000_000 - amount1
    );

    // Duplicate claim must fail
    let duplicate_claim = setup
        .contract_client
        .try_claim(&user1, &amount1, &proof1);
    assert_eq!(duplicate_claim, Err(Ok(AirdropError::AlreadyClaimed)));

    // User 2 claims
    setup.contract_client.claim(&user2, &amount2, &proof2);
    assert!(setup.contract_client.is_claimed(&user2));
    assert_eq!(setup.token_client.balance(&user2), amount2);
}

#[test]
fn test_claim_invalid_proof() {
    let setup = TestSetup::new();
    let user1 = Address::generate(&setup.env);
    let amount1: i128 = 100_000_000;

    let leaf1 = AirdropContract::compute_leaf(setup.env.clone(), user1.clone(), amount1);
    let fake_leaf = BytesN::from_array(&setup.env, &[99u8; 32]);
    let root = hash_pair(&setup.env, &leaf1, &fake_leaf);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);
    setup
        .token_admin_client
        .mint(&setup.contract_id, &1_000_000_000);

    // Wrong proof
    let mut wrong_proof = Vec::new(&setup.env);
    wrong_proof.push_back(BytesN::from_array(&setup.env, &[123u8; 32]));

    let res = setup
        .contract_client
        .try_claim(&user1, &amount1, &wrong_proof);
    assert_eq!(res, Err(Ok(AirdropError::InvalidProof)));
}

#[test]
fn test_claim_wrong_amount() {
    let setup = TestSetup::new();
    let user1 = Address::generate(&setup.env);
    let user2 = Address::generate(&setup.env);
    let amount1: i128 = 100_000_000;
    let amount2: i128 = 200_000_000;

    let leaf1 = AirdropContract::compute_leaf(setup.env.clone(), user1.clone(), amount1);
    let leaf2 = AirdropContract::compute_leaf(setup.env.clone(), user2.clone(), amount2);
    let root = hash_pair(&setup.env, &leaf1, &leaf2);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);
    setup
        .token_admin_client
        .mint(&setup.contract_id, &1_000_000_000);

    let mut proof1 = Vec::new(&setup.env);
    proof1.push_back(leaf2);

    // Try claiming with more than eligible
    let wrong_amount = 500_000_000;
    let res = setup
        .contract_client
        .try_claim(&user1, &wrong_amount, &proof1);
    assert_eq!(res, Err(Ok(AirdropError::InvalidProof)));
}

#[test]
fn test_claim_zero_amount() {
    let setup = TestSetup::new();
    let user = Address::generate(&setup.env);
    let root = BytesN::from_array(&setup.env, &[1u8; 32]);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);

    let proof = Vec::new(&setup.env);
    let res = setup.contract_client.try_claim(&user, &0, &proof);
    assert_eq!(res, Err(Ok(AirdropError::ZeroAmount)));
}

#[test]
fn test_admin_withdraw_remaining() {
    let setup = TestSetup::new();
    let root = BytesN::from_array(&setup.env, &[1u8; 32]);

    setup
        .contract_client
        .init(&setup.admin, &setup.token_id, &root);
    setup
        .token_admin_client
        .mint(&setup.contract_id, &1_000_000_000);

    let recipient = Address::generate(&setup.env);
    setup
        .contract_client
        .withdraw_remaining(&recipient, &400_000_000);

    assert_eq!(setup.token_client.balance(&recipient), 400_000_000);
    assert_eq!(
        setup.token_client.balance(&setup.contract_id),
        600_000_000
    );
}

#[test]
fn test_offchain_consistency() {
    use soroban_sdk::String;
    let env = Env::default();
    let addr = Address::from_string(&String::from_str(
        &env,
        "GBX3ZDMGOAJ2A3FXVY3RZWDHTW7YCFTB5NQPXZWDYWG3PLM2SQAXEWOQ",
    ));
    let amount = 1_000_000_000i128;
    let leaf = AirdropContract::compute_leaf(env.clone(), addr, amount);

    // Exact expected SHA256 hex generated by off-chain JS script
    let expected_hex = "0f2158d58bf9d79b18b4642e691f21bcf10622aad57ad06ee6b4f21ade32f429";
    let mut expected_bytes = [0u8; 32];
    for i in 0..32 {
        expected_bytes[i] = u8::from_str_radix(&expected_hex[i * 2..i * 2 + 2], 16).unwrap();
    }
    let expected_leaf = BytesN::from_array(&env, &expected_bytes);
    assert_eq!(leaf, expected_leaf);
}
