#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, vec, Address, Env, IntoVal,
};
use stellar_royalty_splitter::RoyaltySplitterClient;

// Helper: mint `amount` of the test token to `to`
fn mint(env: &Env, token_admin: &token::StellarAssetClient, to: &Address, amount: i128) {
    token_admin.mint(to, &amount);
}

#[test]
fn test_three_way_split() {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy a test token
    let token_admin_addr = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin_addr.clone());
    let token_admin = token::StellarAssetClient::new(&env, &token_id);
    let token_client = token::Client::new(&env, &token_id);

    // Three collaborators
    let artist = Address::generate(&env);
    let musician = Address::generate(&env);
    let animator = Address::generate(&env);

    // Deploy the splitter contract
    let contract_id = env.register_contract(None, stellar_royalty_splitter::RoyaltySplitter);
    let splitter = RoyaltySplitterClient::new(&env, &contract_id);

    // Initialize: 50% / 30% / 20%  (in basis points)
    splitter.initialize(
        &vec![&env, artist.clone(), musician.clone(), animator.clone()],
        &vec![&env, 5_000u32, 3_000u32, 2_000u32],
    );

    // Fund the contract with 1_000 tokens
    mint(&env, &token_admin, &contract_id, 1_000);

    // Distribute
    splitter.distribute(&token_id, &1_000);

    // Verify balances
    assert_eq!(token_client.balance(&artist), 500);
    assert_eq!(token_client.balance(&musician), 300);
    assert_eq!(token_client.balance(&animator), 200);
}

#[test]
fn test_rounding_dust_goes_to_last_collaborator() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin_addr = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin_addr.clone());
    let token_admin = token::StellarAssetClient::new(&env, &token_id);
    let token_client = token::Client::new(&env, &token_id);

    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let c = Address::generate(&env);

    let contract_id = env.register_contract(None, stellar_royalty_splitter::RoyaltySplitter);
    let splitter = RoyaltySplitterClient::new(&env, &contract_id);

    // 33.33% / 33.33% / 33.34%  — intentionally uneven
    splitter.initialize(
        &vec![&env, a.clone(), b.clone(), c.clone()],
        &vec![&env, 3_333u32, 3_333u32, 3_334u32],
    );

    mint(&env, &token_admin, &contract_id, 100);
    splitter.distribute(&token_id, &100);

    // a and b each get 33, c absorbs the remaining 34
    assert_eq!(token_client.balance(&a), 33);
    assert_eq!(token_client.balance(&b), 33);
    assert_eq!(token_client.balance(&c), 34);
}

#[test]
#[should_panic(expected = "shares must sum to 10000 basis points")]
fn test_invalid_shares_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let a = Address::generate(&env);
    let b = Address::generate(&env);

    let contract_id = env.register_contract(None, stellar_royalty_splitter::RoyaltySplitter);
    let splitter = RoyaltySplitterClient::new(&env, &contract_id);

    // 60% + 60% = 120% — should panic
    splitter.initialize(
        &vec![&env, a, b],
        &vec![&env, 6_000u32, 6_000u32],
    );
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_double_initialization_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let a = Address::generate(&env);
    let b = Address::generate(&env);

    let contract_id = env.register_contract(None, stellar_royalty_splitter::RoyaltySplitter);
    let splitter = RoyaltySplitterClient::new(&env, &contract_id);

    splitter.initialize(
        &vec![&env, a.clone(), b.clone()],
        &vec![&env, 5_000u32, 5_000u32],
    );

    // Second call must panic
    splitter.initialize(
        &vec![&env, a, b],
        &vec![&env, 5_000u32, 5_000u32],
    );
}
