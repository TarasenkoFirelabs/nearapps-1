use std::collections::HashMap;

use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk::{env, PendingContractTx};
use near_sdk_sim::{
    call, deploy, init_simulator, lazy_static_include::syn::token::Use, to_yocto, view,
    ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

extern crate app;
use app::{ContractArgs, NearAppsContract};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_BYTES => "../res/app.wasm",
    TEST_FILE_BYTES => "tests/status_message.wasm",
}

fn init() -> (
    UserAccount,
    ContractAccount<NearAppsContract>,
    UserAccount,
    UserAccount,
) {
    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_limit = u64::MAX;
    genesis.gas_price = 0;
    let master_account = init_simulator(Some(genesis));
    let contract_account = deploy! {
        contract: NearAppsContract,
        contract_id: "contract",
        bytes: &CONTRACT_BYTES,
        signer_account: master_account
    };

    let alice = master_account.create_user(
        AccountId::new_unchecked("alice".to_string()),
        to_yocto("10000"),
    );
    let status = alice.deploy(&TEST_FILE_BYTES, "status".parse().unwrap(), to_yocto("35"));
    (master_account, contract_account, alice, status)
}

#[test]
fn simulate_successful_call() {
    let (master_account, near_apps, _alice, _status) = init();
    let status_id: near_sdk::AccountId = "status".parse().unwrap();
    let status_amt = to_yocto("35");
    let message = "{\"message\": \"hello world\"}";
    let res = call!(
        near_apps.user_account,
        near_apps.add_contract(status_id.clone()),
        gas = DEFAULT_GAS
    );
    println!("SIMPLE CALL: {:#?}", res.promise_results());

    let res = view!(near_apps.print_required_tags());
    println!("Required tags: {:#?}", res.logs());

    let mut map = HashMap::new();
    map.insert("person".to_string(), "Mike".to_string());
    map.insert("company".to_string(), "Near.org".to_string());
    map.insert("purpose".to_string(), "testing123".to_string());
    let arr = vec![map];
    let res = call!(
        master_account,
        near_apps.call(
            arr,
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    println!("COMPLEX CALL: {:#?}", res.promise_results());
    assert!(res.is_ok());
}

#[test]
fn simulate_fail_call() {
    let (master_account, near_apps, _alice, _status) = init();
    let status_id: near_sdk::AccountId = "status".parse().unwrap();
    let status_amt = to_yocto("35");
    let message = "{\"message\": \"hello world\"}";
    call!(
        near_apps.user_account,
        near_apps.add_contract(status_id.clone()),
        gas = DEFAULT_GAS
    );

    let res = call!(
        master_account,
        near_apps.call(
            Vec::new(),
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    assert!(!res.is_ok());

    let mut map = HashMap::new();
    map.insert("person".to_string(), "Mike".to_string());
    map.insert("company".to_string(), "Near.org".to_string());
    //map.insert("purpose".to_string(), "testing123".to_string()); // missing key
    let arr = vec![map];
    let res = call!(
        master_account,
        near_apps.call(
            arr,
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    assert!(!res.is_ok());
}
