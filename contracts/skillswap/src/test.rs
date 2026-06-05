#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

#[test]
fn test_create_and_update_profile() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let teach = String::from_str(&env, "Rust");
    let learn = String::from_str(&env, "Stellar");

    assert_eq!(client.has_profile(&user), false);

    let profile = client.create_profile(&user, &teach, &learn);

    assert_eq!(profile.user, user);
    assert_eq!(profile.teach, teach);
    assert_eq!(profile.learn, learn);
    assert_eq!(profile.active, true);
    assert_eq!(client.has_profile(&user), true);

    let new_teach = String::from_str(&env, "Soroban");
    let new_learn = String::from_str(&env, "Smart Contracts");

    let updated = client.update_profile(&user, &new_teach, &new_learn);

    assert_eq!(updated.teach, new_teach);
    assert_eq!(updated.learn, new_learn);
    assert_eq!(updated.active, true);

    let saved = client.get_profile(&user);

    assert_eq!(saved.teach, new_teach);
    assert_eq!(saved.learn, new_learn);
}

#[test]
fn test_delete_profile() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let teach = String::from_str(&env, "Blockchain");
    let learn = String::from_str(&env, "Rust");

    client.create_profile(&user, &teach, &learn);

    assert_eq!(client.has_profile(&user), true);

    let deleted = client.delete_profile(&user);

    assert_eq!(deleted, true);
    assert_eq!(client.has_profile(&user), false);

    let saved = client.get_profile(&user);

    assert_eq!(saved.active, false);
}