use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount, to_yocto, STORAGE_AMOUNT};
use api::CallContract;
//use crate::utils::init;
use std::str;
use near_sdk::serde_json::json;
use near_sdk::AccountId;

extern crate base64;
use base64::encode;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../res/api_call.wasm",
}

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<CallContract>, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = deploy!(
        contract: CallContract,
        contract_id: CONTRACT_ID,
        bytes: &COUNTER_BYTES,
        signer_account: root
    );

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
fn simulate_log_analytics() {
    let (root, contract, _) = init();

    let initial = "ABC_DEFG_1233_234".to_string();
    let encoded: String = encode(initial.clone());
    let result = call!(
        root,
        contract.log_analytics(encoded)
    );
    result.assert_success();

    let decoded: String = (*result.logs()[0]).to_string(); 

    assert_eq!(initial, decoded);
}
