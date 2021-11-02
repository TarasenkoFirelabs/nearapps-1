use crate::common::Ownable;
use crate::common::StorageKey;
use crate::series::NftSeriesSale;
use chrono::{DateTime, Timelike, Utc};
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_contract_standards::non_fungible_token::metadata::NonFungibleTokenMetadataProvider;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::ValidAccountId;
use std::convert::TryFrom;
use std::time::{Duration, UNIX_EPOCH};

use near_sdk::Balance;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AppContract {
    tokens: NonFungibleToken,
    owner_id: AccountId,
    metadata: LazyOption<NFTContractMetadata>,
    pub pending_nft_rewards: LookupMap<AccountId, Balance>,

    pub series: NftSeriesSale,

    allowed_upgrades: UnorderedSet<UpgradeAllowance>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UpgradeAllowance {
    pub time: (u128, u128),
    pub allowed: bool,
}

impl UpgradeAllowance {
    pub fn new(time: (u128, u128), allowed: bool) -> Self {
        Self { time, allowed }
    }

    pub fn set_time(&mut self, time: (u128, u128), allow: bool) {
        self.time = time;
        self.allowed = allow;
    }
}

#[ext_contract(ext_nearapps)]
trait ExtNearApps {
    fn call(tags: String, contract_name: ValidAccountId, method_name: String, args: String);
}

#[near_bindgen]
impl AppContract {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Upgrade NFT component".to_string(),
                symbol: "UPNFTCOMP".to_string(),
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
            owner_id: owner_id.to_string(),
            pending_nft_rewards: LookupMap::new(b"r"),
            series: NftSeriesSale::new(),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            allowed_upgrades: UnorderedSet::new(b"u"),
        }
    }

    pub fn call_near_apps(self, time: U128, wallet: ValidAccountId, tags: String) {
        if check_correct_time(time.0 as u64) && self.wallet_contains_correct_nft(wallet.clone()) {
            ext_nearapps::call(
                tags,
                ValidAccountId::try_from(env::current_account_id()).unwrap(), // or who we call
                "method_name".to_string(),                                    // unknown yet
                "args".to_string(),                                           // unknown yet
                &env::current_account_id(),
                0,
                env::prepaid_gas() / 3, // need to test how much gas is needed
            );
        }
    }

    #[payable]
    pub fn refund_deposit(&mut self, storage_used: u64, extra_spend: Balance) {
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

    fn wallet_contains_correct_nft(self, wallet: ValidAccountId) -> bool {
        self.nft_supply_for_owner(wallet.clone()).0 > 0
            && wallet.as_ref() == &env::predecessor_account_id()
    }

    pub fn set_allowed_timelapse(&mut self, start_time: U128, end_time: U128) {
        self.assert_owner();
        if start_time.0 > end_time.0 {
            env::panic(b"start time is bigger, than endtime");
        }
        self.allowed_upgrades
            .insert(&UpgradeAllowance::new((start_time.0, end_time.0), true));
    }

    fn use_allowance(&mut self, time: u128) {
        let item = self.allowed_upgrades.iter().find(|allowance| {
            allowance.allowed && allowance.time.0 < time && allowance.time.1 > time
        });
        if let Some(item) = item {
            self.allowed_upgrades.remove(&item);
        }
    }
}

impl Ownable for AppContract {
    fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
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

near_contract_standards::impl_non_fungible_token_core!(AppContract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(AppContract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(AppContract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for AppContract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
