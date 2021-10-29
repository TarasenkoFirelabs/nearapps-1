use crate::series::NftSeriesSale;
use crate::series::NftSeriesId;
use crate::common::{StorageKey};
use chrono::{DateTime, Timelike, Utc};
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::ValidAccountId;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::test_utils::test_env::bob;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AppContract {
    tokens: NonFungibleToken,
    owner_id: ValidAccountId,
    metadata: LazyOption<NFTContractMetadata>,
    pending_nft_rewards: LookupMap<AccountId, Balance>,
    
    pub series: NftSeriesSale,
}

#[ext_contract(ext_nearapps)]
trait ExtNearApps {
    fn call(tags: String, contract_name: ValidAccountId, contract_inputs: String);
}


#[near_bindgen]
impl AppContract {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Comic by Paras".to_string(),
                symbol: "COMIC".to_string(),
                icon: None,
                base_uri: Some("https://ipfs.fleek.co/ipfs".to_string()),
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            owner_id,
            pending_nft_rewards: LookupMap::new(b"r"),
            series:NftSeriesSale::new(),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }


    pub fn call_near_apps(time: U128, tags: String) { //  wallet: ValidAccountId ? or we just get predecessor/signer account_id
        if check_correct_time(time.0 as u64) && wallet_contains_correct_nft(env::predecessor_account_id()) {
            //ext_nearapps::call(tags, ValidAccountId::try_from("contract.name").unwrap(), "contract_inputs".to_string(), &env::current_account_id(), 0, env::prepaid_gas() / 3);
        }
    }

    #[payable]
    pub fn refund_deposit(&mut self,storage_used: u64, extra_spend: Balance) {
        let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
        let attached_depo = env::attached_deposit() - extra_spend;

        assert!(
            required_cost <= attached_depo,
            "Must attach {} some yocto to cover storage",
            required_cost,
        );

        let refund = attached_depo - required_cost;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

fn check_correct_time(time: u64) -> bool {
    let d = UNIX_EPOCH + Duration::from_secs(time);
    let datetime = DateTime::<Utc>::from(d);
    let hour = datetime.hour();
    match hour {
        0..=6 | 22..=23 => true,
        _ => env::panic("bad time".as_bytes()),
    }
}

fn wallet_contains_correct_nft(wallet: String) -> bool {
    true
}
