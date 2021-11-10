use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::ValidAccountId;
use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};
use std::convert::TryFrom;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SendNFTContract {
    nft: NonFungibleToken,
    owner_id: AccountId,
    tokens: UnorderedSet<(AccountId, TokenId)>,
}
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
#[near_bindgen]
impl Ownable for SendNFTContract {
    fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn nft_transfer(&mut self, account_id: AccountId, token_id: TokenId) -> Promise;
    fn nft_transfer_callback(&mut self) -> Promise;
    fn nft_batch_transfer(&mut self)-> Promise;
}

const GAS_FOR_ROYALTIES: Gas = 0;
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl SendNFTContract {
    #[init]
    pub fn new(owner_id: AccountId, mappings: Vec<(AccountId, TokenId)>) -> Self {
        let owner = ValidAccountId::try_from(owner_id.clone()).expect("Invalid AccountId");
        assert!(!env::state_exists(), "Already initialized");
        let mut nft = NonFungibleToken::new(
            StorageKey::NonFungibleToken,
            owner.clone(),
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        for (_, token_id) in &mappings {
            nft.mint(token_id.into(), owner.clone(), None);
        }
        let mut map = UnorderedSet::new(StorageKey::TokenAccountMapping);
        map.extend(mappings);
        Self {
            owner_id,
            tokens: map,
            nft,
        }
    }
    
    pub fn nft_batch_transfer(&mut self) ->bool {
        self.assert_owner();
        for (acc_id, t_id) in self.tokens.iter() {
            //self.nft.nft_transfer_call(, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>, msg: String)
            ext_self::nft_transfer(
                acc_id,
                t_id,
                &env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_ROYALTIES,
            )
            .then(ext_self::nft_transfer_callback(
                &env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_ROYALTIES,
            ));
        }
        true
    }
    #[private]
    pub fn nft_transfer(&mut self, account_id: AccountId, token_id: TokenId) {
        //let account = ValidAccountId::try_from(account_id.clone()).unwrap();

        self.nft.internal_transfer_unguarded(&token_id, &self.owner_id, &account_id);
    }
    #[private]
    pub fn nft_transfer_callback(&mut self) -> bool {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => true,
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nft_batch_transfer() {
        assert_eq!(2 + 2, 4);
    }
}
