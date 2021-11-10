use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount, to_yocto, DEFAULT_GAS};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use std::str;
//use core::convert::TryFrom;
use std::convert::{TryFrom, TryInto};
use near_sdk::json_types::ValidAccountId;

extern crate app_nft;
use app_nft::{NftContractContract};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    AIRDROP_BYTES => "res/app_nft.wasm",
}

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<NftContractContract>, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = deploy!(
        contract: NftContractContract,
        contract_id: CONTRACT_ID,
        bytes: &AIRDROP_BYTES,
        signer_account: root
    );

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
//#[should_panic(expected = "Ownable: predecessor is not the owner")]
fn simulate_airdrop_default_meta() {
    let (root, contract, alice) = init();

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    let res = call!(
        root, contract.new_default_meta(valid_account.to_string()));

    let token_meta = TokenMetadata{
        title: Some("TestMetadata".to_string()),
        description: None, 
        media: None, 
        media_hash: None,
        copies: None, 
        issued_at: None, 
        expires_at: None, 
        starts_at: None,
        updated_at: None, 
        extra: None, 
        reference: None, 
        reference_hash: None, 
    };

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    let res = call! {
        root,
        contract.nft_mint("New_test_token".to_string(), valid_account, token_meta),
        gas = DEFAULT_GAS
    };


}