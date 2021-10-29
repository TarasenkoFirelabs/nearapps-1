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
use std::collections::HashMap;

use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AppContract {
    tokens: NonFungibleToken,
    owner_id: ValidAccountId,
    metadata: LazyOption<NFTContractMetadata>,

    pending_nft_rewards: LookupMap<AccountId, Balance>,

    token_series_by_id: UnorderedMap<NftSeriesId,NftSeriesSale>,
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
            token_series_by_id: UnorderedMap::new(StorageKey::TokenSeriesById),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    near_contract_standards::impl_fungible_token_core!(Contract, token);
    near_contract_standards::impl_fungible_token_storage!(Contract, token);
}
