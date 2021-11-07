mod internal;
mod mint;

use crate::internal::*;
use near_contract_standards::non_fungible_token::{core::NonFungibleTokenCore, NonFungibleToken};
use near_contract_standards::non_fungible_token::{
    metadata::{
        NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
    },
    Token, TokenId,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    collections::LazyOption, env, ext_contract, near_bindgen, AccountId, BorshStorageKey,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};


use std::convert::*;
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NftContract {
    token: NonFungibleToken,
    owner_id: AccountId,
    total_supply: u128,
    metadata: LazyOption<NFTContractMetadata>,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    TokenAccountMapping,
    TokenMetadata,
    Enumeration,
    Approval,
    Metadata,
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

#[ext_contract(ext_self)]
trait ExtSelf {
    fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        metadata: TokenMetadata,
    ) -> Promise;
}

const GAS_FOR_ROYALTIES: Gas = 0;
const NO_DEPOSIT: Balance = 0;

near_contract_standards::impl_non_fungible_token_core!(NftContract, token);
near_contract_standards::impl_non_fungible_token_approval!(NftContract, token);
near_contract_standards::impl_non_fungible_token_enumeration!(NftContract, token);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for NftContract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl NftContract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
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
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert_initialized();
        metadata.assert_valid();
        let owner = ValidAccountId::try_from(owner_id.clone()).expect("Invalid AccountId");

        let mut nft = NonFungibleToken::new(
            StorageKey::NonFungibleToken,
            owner,
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        Self {
            owner_id,
            token: nft,
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            total_supply: 0,
        }
    }
}
