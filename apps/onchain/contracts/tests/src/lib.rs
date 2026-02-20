#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String, Symbol,
};

// 1. Bytes only - No contractimport! to avoid DataKey collisions
mod wasm_files {
    pub const TOKEN: &[u8] = include_bytes!("../../../target/wasm32v1-none/release/lumen_token.wasm");
    pub const REGISTRY: &[u8] = include_bytes!("../../../target/wasm32v1-none/release/contributor_registry.wasm");
    pub const VAULT: &[u8] = include_bytes!("../../../target/wasm32v1-none/release/crowdfund_vault.wasm");
}

// 2. Import Clients from the actual contract crates
use lumen_token::{LumenTokenClient as TokenClient};
use contributor_registry::{ContributorRegistryContractClient as RegistryClient};
use crowdfund_vault::{CrowdfundVaultContractClient as VaultClient};

#[test]
fn test_lumenpulse_protocol_e2e() {
    let env = Env::default();
    env.mock_all_auths();

    // Setup Identities
    let admin = Address::generate(&env);
    let contributor = Address::generate(&env);
    let project_owner = Address::generate(&env);

    // 3. Registering using the non-deprecated .register method
    // Note: Soroban now uses .register(WASM_BYTES, ()) for WASM registration
    let token_id = env.register(wasm_files::TOKEN, ());
    let reg_id = env.register(wasm_files::REGISTRY, ());
    let vault_id = env.register(wasm_files::VAULT, ());

    // 4. Create Clients
    let token_client = TokenClient::new(&env, &token_id);
    let reg_client = RegistryClient::new(&env, &reg_id);
    let vault_client = VaultClient::new(&env, &vault_id);

    // 5. Initialize Protocol
    token_client.initialize(&admin, &7u32, &String::from_str(&env, "Lumen"), &String::from_str(&env, "LUM"));
    reg_client.initialize(&admin);
    vault_client.initialize(&admin);

    // 6. Full Flow: Register -> Mint -> Project -> Deposit
    reg_client.register_contributor(&contributor, &String::from_str(&env, "cedarich"));
    token_client.mint(&contributor, &10000i128);

    let project_id = vault_client.create_project(
        &project_owner, 
        &Symbol::new(&env, "DevTools"), 
        &5000i128, 
        &token_id
    );

    vault_client.deposit(&contributor, &project_id, &3000i128);

    // 7. Assertions
    assert_eq!(token_client.balance(&contributor), 7000i128);
    assert_eq!(vault_client.get_balance(&project_id), 3000i128);

    // Milestone & Withdrawal
    vault_client.approve_milestone(&admin, &project_id);
    vault_client.withdraw(&project_id, &2000i128);

    assert_eq!(token_client.balance(&project_owner), 2000i128);
    
    std::println!("✅ Integration Test Passed!");
}