use near_sdk_sim::{
    call, deploy, init_simulator, lazy_static_include::syn::token::Use, to_yocto, view,
    ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_WASM_BYTES => "res/app_wallet_creation.wasm",
}

use app_wallet_creation::{MakeWalletsContract, NewAccount};

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, UserAccount, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = root.deploy(
        &CONTRACT_WASM_BYTES,
        CONTRACT_ID.to_string(),
        STORAGE_AMOUNT // attached deposit
    );

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}