use std::str::FromStr;

use near_sdk::{PublicKey, json_types::U128};
use near_sdk_sim::{
    call, deploy, init_simulator, lazy_static_include::syn::token::Use, to_yocto, view,
    ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_BYTES => "res/app_wallet_creation.wasm",
}

use app_wallet_creation::{MakeWalletsContract, NewAccount};

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<MakeWalletsContract>, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = deploy!{
        contract: MakeWalletsContract,
        contract_id: CONTRACT_ID,
        bytes: &CONTRACT_BYTES,
        signer_account: root
    };

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
pub fn basic(){
    let (root, contract, user) = init();

    let account_id = "adsick".parse().unwrap();
    let public_key = PublicKey::from_str("ed25519:8MtAwUtEuU18u9xrehUEBWgcziTHxFhXLNE9F5xq7ExU").unwrap();
    let initial_amount = U128::from(to_yocto("1"));
    let new_account = NewAccount::new(account_id, public_key, initial_amount);

    // uses default gas amount
    let result = call!(user, contract.make_wallets(new_account), deposit = to_yocto("10").into());
    println!("{:?}", result.outcome().logs);
    println!("{:?}", result.outcome().status);
    assert!(result.is_ok()); //fails

}
