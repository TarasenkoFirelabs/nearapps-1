use std::collections::HashMap;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::collections::LookupMap;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::collections::UnorderedMap;

use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
    
};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    TokenAccountMapping,
    TokenMetadata,
    Enumeration,
    Approval,
}
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
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AirdropRewards(Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct AirdropReward {
    account_id: AccountId,
    amount: Balance,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AppContract {
    token: NonFungibleToken,
    owner: AccountId,
    pending_ft_rewards: LookupMap<AccountId, Balance>,
}
pub trait SupportsAirdrop{
 fn airdrop(&mut self, rewards: AirdropRewards);
 fn add_pending_ft_rewards(&mut self, rewards: Vec<(AccountId, Balance)>);
 fn get_pending_ft_rewards() -> LookupMap<AccountId, Balance>;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LinkDrop {
    pub accounts: UnorderedMap<AccountId, Balance>,
}
pub type NftSeriesId = String;

//Nft Series
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftSeries {
	metadata: TokenMetadata,
	creator_id: AccountId,
	tokens: UnorderedSet<TokenId>,
    price: Option<Balance>,
    is_mintable: bool,
    royalty: HashMap<AccountId, u32>
}