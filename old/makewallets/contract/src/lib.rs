use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Promise, *};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MakeWallets {}

impl Default for MakeWallets {
    fn default() -> Self {
        Self {}
    }
}

const GAS: u64 = 40_000_000_000_000;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NewAccount {
    account_id: AccountId,
    public_key: PublicKey,
    initial_amount: U128,
}

#[ext_contract(ext_self)]
pub trait ExtMakeWallets {
    fn on_account_created(#[callback] val: bool) -> bool;
}

#[near_bindgen]
impl MakeWallets {
    #[payable]
    pub fn make_wallets(new_account: NewAccount) {
        Promise::new("near".parse().unwrap()).function_call(
                "create_account".to_string(),
                json!({"new_account_id": new_account.account_id.to_string(), "new_public_key": new_account.public_key}).to_string().as_bytes().to_vec(),
                new_account.initial_amount.0, //initial deposit
                GAS.into()
           ).then(ext_self::on_account_created(env::current_account_id(), 0, GAS.into()));
           
    }

    #[private]
    pub fn on_account_created(#[callback] val: bool) {
        match val {
            true => env::log_str("account was created successfully"),
            false => env::log_str("error during account creation"),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    // use super::*;
    // use near_sdk::{testing_env, VMContext};

    // fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    //     VMContext {
    //         current_account_id: "alice_near".to_string(),
    //         signer_account_id: "bob_near".to_string(),
    //         signer_account_pk: vec![0, 1, 2],
    //         predecessor_account_id: "carol_near".to_string(),
    //         input,
    //         block_index: 0,
    //         block_timestamp: 0,
    //         account_balance: 0,
    //         account_locked_balance: 0,
    //         storage_usage: 0,
    //         attached_deposit: 0,
    //         prepaid_gas: 10u64.pow(18),
    //         random_seed: vec![0, 1, 2],
    //         is_view,
    //         output_data_receivers: vec![],
    //         epoch_height: 0,
    //     }
    // }

    // #[test]
    // fn create_accounts() {
    //     let context = get_context(vec![], false);
    //     testing_env!(context);
    //     let mut contract = MakeWallets::default();
    // }
}
