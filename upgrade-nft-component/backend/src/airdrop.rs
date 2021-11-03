use crate::appcontract::{AppContract, AppContractContract};
use crate::common::{AirdropRewards, Ownable, SupportsAirdrop};

//use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};
//use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::Balance;

#[near_bindgen]
impl SupportsAirdrop for AppContract {
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, Balance)>) {
        self.assert_owner();
        for reward in rewards {
            let account_id = reward.0.to_string();
            let balance = reward.1;
            //let prev = self.pending_nft_rewards.get(&account_id).unwrap_or_default();
            //if let Some(res) = u128::checked_add(prev, reward.1) {
            //    self.pending_nft_rewards.insert(&account_id, &res);
            //} else {
            //    panic!("Error");
            //}
            self.pending_nft_rewards.insert(&account_id, &balance);
        }
    }

    #[payable]
    fn airdrop(&mut self, rewards: AirdropRewards) {
        self.assert_owner();
        for reward in rewards.0 {
            let account = reward.account_id.to_string();
            self.tokens.internal_transfer_unguarded(&reward.token_id,&self.owner(), &account);
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    //fn get_context(predecessor: AccountId) -> VMContextBuilder {
    //    let mut builder = VMContextBuilder::new();
    //    builder.predecessor_account_id(predecessor);
    //    builder
    //}

    // TESTS HERE
}
