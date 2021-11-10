use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::Serialize;
use std::collections::HashMap;

use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner(),
            "Ownable: predecessor is not the owner"
        );
    }
    fn owner(&self) -> AccountId;
    fn transfer_ownership(&mut self, owner: AccountId);
}
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropRewards(pub Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropReward {
    pub account_id: AccountId,
    pub amount: Balance,
    pub token_id:TokenId,
}

pub trait SupportsAirdrop {
    fn airdrop(&mut self, rewards: AirdropRewards);
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, Balance)>);
    //fn get_pending_ft_rewards() -> LookupMap<AccountId, Balance>;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LinkDrop {
    pub accounts: UnorderedMap<AccountId, Balance>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    TokenAccountMapping,
    TokenMetadata,
    Enumeration,
    Approval,
    TokenSeriesById,
    TokensBySeriesInner { identifier: String },
    Metadata,
}
