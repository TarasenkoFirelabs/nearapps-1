use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount};
//use crate::utils::init;
use std::str;
//use near_sdk::serde_json::json;

extern crate base64;
use base64::encode;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../res/api_call.wasm",
}

#[test]
fn simulate_log_analytics() {
    let (root, contract, _, _, _) = init();

    let initial = String::from("ABC_DEFG_1233");
    let encoded: String = encode(initial);
    let decoded: String = root.view(
        contract.log_analytics(encoded)
    );

    assert_eq!(initial, decoded);
}
