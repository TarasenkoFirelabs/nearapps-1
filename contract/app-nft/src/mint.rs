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
        let token=self.token.mint(token_id, receiver_id, Some(metadata));
        self.total_supply += 1;
        
        token.token_id
    }
}
