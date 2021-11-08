use near_contract_standards::non_fungible_token::TokenId;
use crate::*;
use near_sdk::{AccountId};
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropRewards(pub Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropReward {
    pub account_id: AccountId,
    pub token_id:TokenId,
}

pub trait SupportsAirdrop {
    fn airdrop(&mut self, rewards: AirdropRewards);
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>);
    //fn get_pending_ft_rewards() -> LookupMap<AccountId, Balance>;
}

impl SupportsAirdrop for NftContract {
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>) {
        self.assert_owner();
        for reward in rewards {
            let account_id = reward.0.to_string();
            let token_id = reward.1;
            self.pending_nft_rewards.insert(&account_id, &token_id);
        }
    }

    fn airdrop(&mut self, rewards: AirdropRewards) {
        self.assert_owner();
        for reward in rewards.0 {
            let account = reward.account_id.to_string();
            self.nft_transfer_unsafe(&reward.token_id, &self.owner(), &account);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    #[should_panic(expected = "Ownable: predecessor is not the owner")]
    fn test_airdrop_default_meta_panic() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        //let valid_account = TryFrom::try_from("test_airdop_owner.testnet").unwrap();
        let account_id:AccountId="test_airdop_owner.testnet".into();
        let mut contract = NftContract::new_default_meta(account_id);
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.testnet".to_string(),
            token_id: "token".to_string(),
        };
        let token_id=reward.token_id.clone();
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![("test_airdop_owner.testnet".to_string(), token_id)]);
        contract.airdrop(rewards);
    }

    #[test]
    fn test_airdrop_default_meta() {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(TryFrom::try_from("test_airdop_owner.testnet").unwrap());
        testing_env!(context.build());

        //let valid_owner: ValidAccountId = TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap();
        let owner = "test_airdop_owner.testnet".to_string();
        //let valid_owner: ValidAccountId = TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap();
        let mut contract = NftContract::new_default_meta(
            TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap(),
        );
        println!(
            "{}",
            &contract
                .token
                .tokens_per_owner
                .as_ref()
                .unwrap()
                .contains_key(&owner)
        );
        let token_id = (&contract.token.tokens_per_owner.as_ref().unwrap())
            .get(&owner)
            .unwrap()
            .to_vec()[0]
            .clone();
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.testnet".to_string(),
            token_id: token_id.clone(),
        };
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![(owner, token_id)]);
        contract.airdrop(rewards);
    }
}

