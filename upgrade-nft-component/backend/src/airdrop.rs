use crate::appcontract::{AppContract, AppContractContract};
use crate::common::{AirdropReward, AirdropRewards, Ownable, SupportsAirdrop};

use near_sdk::{near_bindgen, AccountId};
use near_sdk::Balance;

#[near_bindgen]
impl SupportsAirdrop for AppContract {
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, Balance)>) {
        self.assert_owner();
        for reward in rewards {
            let account_id = reward.0.to_string();
            let balance = reward.1;
            self.pending_nft_rewards.insert(&account_id, &balance);
        }
    }

    #[payable]
    fn airdrop(&mut self, rewards: AirdropRewards) {
        self.assert_owner();
        for reward in rewards.0 {
            let account = reward.account_id.to_string();
            self.tokens.internal_transfer_unguarded(&reward.token_id, &self.owner(), &account);
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    //#[test]
    #[should_panic(expected = "Ownable: predecessor is not the owner")]
    fn test_airdrop_default_meta_panic() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        
        let valid_account = TryFrom::try_from("test_airdop_owner.testnet").unwrap();
        let mut contract = AppContract::new_default_meta(valid_account);
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.testnet".to_string(),
            amount: 0,
            token_id: "token".to_string(),
        };
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![("test_airdop_owner.testnet".to_string(), 0)]);
        contract.airdrop(rewards);
    }

    #[test]
    fn test_airdrop_default_meta() {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(TryFrom::try_from("test_airdop_owner.testnet").unwrap());
        testing_env!(context.build());

        //let valid_owner: ValidAccountId = TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap();
        let owner = "test_airdop_owner.testnet".to_string();
        let receiver = "test_airdop_receiver.testnet".to_string();
        let mut contract = AppContract::new_default_meta(TryFrom::try_from(owner.clone()).unwrap());
        contract.add_pending_rewards(vec![(receiver.clone(), 0)]);
        println!("{}", contract.pending_nft_rewards.contains_key(&receiver));
        println!("{}", &contract.tokens.tokens_per_owner.as_ref().unwrap().contains_key(&owner));
        //let token_id = (&contract.tokens.tokens_per_owner.as_ref().unwrap()).get(&owner).unwrap().to_vec()[0].clone();
        //let token_id = 
        let reward = AirdropReward {
            account_id: receiver.clone(),
            amount: 0,
            token_id: token_id,
        };
        let rewards = AirdropRewards(vec![reward]);
        contract.airdrop(rewards);
    }
}
