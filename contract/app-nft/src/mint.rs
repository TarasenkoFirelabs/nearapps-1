use crate::*;
use near_sdk::collections::UnorderedSet;
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
        let token = self.token.mint(token_id, receiver_id, Some(metadata));
        self.total_supply +=1;

        token.token_id
    }

    fn nft_create_series(
        &mut self,
        series_id: NftSeriesId,
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
            "Near Apps: royalty exceeds 10 accounts"
        );

        assert!(
            total <= 9000,
            "Near Apps Exceeds maximum royalty -> 9000",
        );

        let price_res: Option<u128> = if price.is_some() {
            Some(price.unwrap())
        } else {
            None
        };

        self.token_series.insert(
            &series_id,
            &NftSeries {
                series_id: (*&series_id).clone(),
                metadata: token_metadata.clone(),
                creator_id: creator_id.to_string(),
                tokens: UnorderedSet::new(
                    StorageKey::TokensBySeriesInner {
                        identifier: series_id.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                ),
                closed: false,
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

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        NftSeriesJson {
            series_id,
            metadata: token_metadata,
            creator_id: creator_id.into(),
            royalty: royalty_res,
        }
    }

    fn nft_mint_series_internal(
        &mut self, 
        token_series_id: NftSeriesId, 
        receiver_id: ValidAccountId
    ) -> TokenId {
        let mut token_series = self.token_series.get(&token_series_id).expect("Near Apps: Token series not exist");
        assert!(
            token_series.is_mintable,
            "Near Apps: Token series is not mintable"
        );

        let num_tokens = token_series.tokens.len();
        let max_copies = token_series.metadata.copies.unwrap_or(u64::MAX);
        assert!(num_tokens < max_copies, "Series supply maxed");

        if (num_tokens + 1) >= max_copies {
            token_series.is_mintable = false;
        }

        let token_id = format!("{}{}{}", &token_series_id, TOKEN_DELIMETER, num_tokens + 1);
        token_series.tokens.insert(&token_id);
        self.token_series.insert(&token_series_id, &token_series);

        // you can add custom metadata to each token here
        let metadata = Some(TokenMetadata {
            title: None,          // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
            description: None,    // free-form description
            media: None, // URL to associated media, preferably to decentralized, content-addressed storage
            media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
            copies: None, // number of copies of this set of metadata in existence when token was minted.
            issued_at: Some(env::block_timestamp().to_string()), // ISO 8601 datetime when token was issued or minted
            expires_at: None, // ISO 8601 datetime when token expires
            starts_at: None, // ISO 8601 datetime when token starts being valid
            updated_at: None, // ISO 8601 datetime when token was last updated
            extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
            reference: None, // URL to an off-chain JSON file with more info.
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        });

        //let token = 
        self.token.mint(token_id.clone(), receiver_id, metadata);
        
        token_id
    }
}
