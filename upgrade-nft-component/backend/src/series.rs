use crate::appcontract::AppContract;
use crate::common::{StorageKey};

use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, env, near_bindgen, serde_json::json};
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
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftSeriesJson {
    series_id: NftSeriesId,
    metadata: TokenMetadata,
    creator_id: AccountId,
    royalty: HashMap<AccountId, u32>,
}

pub trait NftSeriesProvider {
    fn nft_create_series(
        &mut self,
        token_metadata: TokenMetadata,
        price: Option<Balance>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> NftSeriesJson;

    fn nft_series_mint(&mut self, series_id: NftSeriesId, receiver_id: AccountId) -> TokenId;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftSeriesSale {
    inner: UnorderedMap<NftSeriesId, NftSeries>,
}
impl NftSeriesSale{
    pub fn new()->Self{
         Self{
             inner:UnorderedMap::new(StorageKey::TokenSeriesById)
         }
    }
}
impl NftSeriesProvider for AppContract {
    
    fn nft_create_series(
        &mut self,
        token_metadata: TokenMetadata,
        price: Option<Balance>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> NftSeriesJson {
        let initial_storage_usage = env::storage_usage();
        let creator_id = env::predecessor_account_id();

        let series_id = format!("{}", (self.series.inner.len() + 1));

        assert!(
            self.series.inner.get(&series_id).is_none(),
            "NearNftComponent: duplicate series_id"
        );

        let title = token_metadata.title.clone();
        assert!(
            title.is_some(),
            "NearNftComponent: token_metadata.title is required"
        );

        let mut total_perpetual = 0;
        let mut total_accounts = 0;
        let royalty_res = if let Some(royalty) = royalty {
            for (_, v) in royalty.iter() {
                total_perpetual += *v;
                total_accounts += 1;
            }
            royalty
        } else {
            HashMap::new()
        };

        assert!(
            total_accounts <= 10,
            "NearNftComponent: royalty exceeds 10 accounts"
        );

        assert!(
            total_perpetual <= 9000,
            "NearNftComponent Exceeds maximum royalty -> 9000",
        );

        let price_res: Option<u128> = if price.is_some() {
            Some(price.unwrap())
        } else {
            None
        };

        self.series.inner.insert(
            &series_id,
            &NftSeries {
                series_id:(*&series_id).clone(),
                metadata: token_metadata.clone(),
                creator_id: creator_id.to_string(),
                tokens: UnorderedSet::new(
                    StorageKey::TokensBySeriesInner {
                        identifier: series_id.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                ),
                closed:false,
                price: price_res,
                is_mintable: true,
                royalty: royalty_res.clone(),
            },
        );

        env::log(
            json!({
                "type": "nft_create_series",
                "params": {
                    "series_id": series_id,
                    "token_metadata": token_metadata,
                    "creator_id": creator_id,
                    "price": price,
                    "royalty": royalty_res
                }
            })
            .to_string()
            .as_bytes(),
        );

        self.refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        NftSeriesJson {
            series_id,
            metadata: token_metadata,
            creator_id: creator_id.into(),
            royalty: royalty_res,
        }
    }
    fn nft_series_mint(&mut self, _: std::string::String, _: std::string::String) -> TokenId {
        todo!()
    }
}
