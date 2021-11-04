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

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-backend' comes from Cargo.toml's 'name' key
 */

#[cfg(test)]
mod tests {
  use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
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
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let valid_account = TryFrom::try_from("bob.near").unwrap();
        let mut contract = AppContract::new_default_meta(valid_account);
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.near".to_string(),
            amount: 0,
            token_id: "token".to_string(),
        };
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![("test_airdop_owner.near".to_string(), 0)]);
        contract.airdrop(rewards);
    }
}
