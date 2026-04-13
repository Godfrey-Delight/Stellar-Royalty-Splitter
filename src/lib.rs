#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, vec,
    Address, Env, Map, Vec,
};

// ── Storage keys ────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Collaborators,  // Vec<Address>
    Shares,         // Map<Address, u32>  (basis points, sum == 10_000)
    Initialized,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct RoyaltySplitter;

#[contractimpl]
impl RoyaltySplitter {
    /// Initialize the contract with collaborator addresses and their share
    /// allocations expressed in basis points (1 bp = 0.01%).
    /// The sum of all shares MUST equal 10_000 (i.e. 100.00%).
    ///
    /// # Arguments
    /// * `collaborators` – ordered list of recipient addresses
    /// * `shares`        – basis-point allocation per collaborator (same order)
    pub fn initialize(
        env: Env,
        collaborators: Vec<Address>,
        shares: Vec<u32>,
    ) {
        // Prevent re-initialization
        if env.storage().instance().has(&DataKey::Initialized) {
            panic!("already initialized");
        }

        let len = collaborators.len();
        assert!(len > 0, "no collaborators");
        assert!(len == shares.len(), "length mismatch");

        // Validate shares sum to 10_000 bp (100%)
        let total: u32 = shares.iter().sum();
        assert!(total == 10_000, "shares must sum to 10000 basis points");

        let mut share_map: Map<Address, u32> = Map::new(&env);
        for i in 0..len {
            let addr = collaborators.get(i).unwrap();
            let bp = shares.get(i).unwrap();
            assert!(bp > 0, "share must be > 0");
            share_map.set(addr, bp);
        }

        env.storage().instance().set(&DataKey::Collaborators, &collaborators);
        env.storage().instance().set(&DataKey::Shares, &share_map);
        env.storage().instance().set(&DataKey::Initialized, &true);
    }

    /// Distribute `amount` of `token` held by this contract to all
    /// collaborators according to their pre-defined shares.
    ///
    /// Anyone can call this; the contract must already hold the funds.
    pub fn distribute(env: Env, token: Address, amount: i128) {
        assert!(amount > 0, "amount must be positive");

        let collaborators: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Collaborators)
            .expect("not initialized");

        let share_map: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Shares)
            .expect("not initialized");

        let token_client = token::Client::new(&env, &token);

        let mut distributed: i128 = 0;
        let last_index = collaborators.len() - 1;

        for (i, addr) in collaborators.iter().enumerate() {
            let bp = share_map.get(addr.clone()).unwrap() as i128;

            // Last collaborator receives the remainder to absorb rounding dust
            let payout = if i as u32 == last_index {
                amount - distributed
            } else {
                amount * bp / 10_000
            };

            if payout > 0 {
                token_client.transfer(&env.current_contract_address(), &addr, &payout);
            }

            distributed += payout;
        }
    }

    // ── View helpers ─────────────────────────────────────────────────────────

    /// Returns all collaborator addresses.
    pub fn get_collaborators(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Collaborators)
            .unwrap_or(vec![&env])
    }

    /// Returns the basis-point share for a single collaborator.
    pub fn get_share(env: Env, collaborator: Address) -> u32 {
        let share_map: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Shares)
            .expect("not initialized");
        share_map.get(collaborator).unwrap_or(0)
    }
}
