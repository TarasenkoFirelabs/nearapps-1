use near_sdk::collections::UnorderedSet;
use crate::*;
#[near_bindgen]
impl NftContract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        metadata: TokenMetadata,
    ) -> TokenId {

        self.assert_owner();
        let token=self.token.mint(token_id, receiver_id, Some(metadata));
        self.total_supply += 1;

        token.token_id
    }

    fn nft_create_series(
        &mut self,
        series_id:NftSeriesId,
        token_metadata: TokenMetadata,
        price: Option<Balance>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> NftSeriesJson {
        let initial_storage_usage = env::storage_usage();
        let creator_id = env::predecessor_account_id();

        assert!(
            self.token_series.get(&series_id).is_none(),
            "NearNftComponent: duplicate series_id"
        );

        let title = token_metadata.title.clone();
        assert!(
            title.is_some(),
            "NearNftComponent: token_metadata.title is required"
        );

        let mut total = 0;
        let mut total_accounts = 0;
        let royalty_res = if let Some(royalty) = royalty {
            for (_, v) in royalty.iter() {
                total += *v;
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
            total <= 9000,
            "NearNftComponent Exceeds maximum royalty -> 9000",
        );

        let price_res: Option<u128> = if price.is_some() {
            Some(price.unwrap())
        } else {
            None
        };

        self.token_series.insert(
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

        refund_deposit(env::storage_usage() - initial_storage_usage,0);

        NftSeriesJson {
            series_id,
            metadata: token_metadata,
            creator_id: creator_id.into(),
            royalty: royalty_res,
        }
    }
    fn nft_series_mint(&mut self, series_id: NftSeriesId, receiver_id: AccountId) -> TokenId{
        todo!()
    }
}
