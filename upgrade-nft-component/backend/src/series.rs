use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::{AccountId, Balance};
use std::collections::HashMap;

pub type NftSeriesId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftSeries {
    series_id: NftSeriesId,
    metadata: TokenMetadata,
    creator_id: AccountId,
    tokens: UnorderedSet<TokenId>,
    price: Option<Balance>,
    is_mintable: bool,
    royalty: HashMap<AccountId, u32>,
    closed: bool,
}

pub trait NftSeriesProvider {
    fn nft_create_series(
        &mut self,
        token_metadata: TokenMetadata,
        price: Option<Balance>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> NftSeries;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftSeriesSale {
    token_series_by_id: UnorderedMap<NftSeriesId, NftSeriesSale>,
}

// impl AppContract {
//     pub fn nft_create_series(
//         &mut self,
//         token_metadata: TokenMetadata,
//         price: Option<Balance>,
//         royalty: Option<HashMap<AccountId, u32>>,
//     ) -> NftSeries{
//      NftSeries{
//          tokens,

//      }
//     }
// }
