use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Promise, *};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MakeWallets {}

impl Default for MakeWallets {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl MakeWallets {
    //falls when I try to view it.
    //idk should it be payable or not, works without it tho
    //note we will create normal accounts, not subaccounts in the future, so String will become AccountId
    pub fn make_wallets(accounts: Vec<String>, amounts: Vec<U128>) -> Promise {
        if accounts.is_empty() {
            env::panic_str("you need to provide account list")
        } else if accounts.len() != amounts.len() {
            env::panic_str("the length list of initial deposits is not equal to list af accounts")
            //todo make better message
        }

        let mut accounts = accounts
            .into_iter()
            .map(|a| AccountId::new_unchecked(format!("{}.{}", a, env::current_account_id()))); //todo delete for not creating subaccounts
        let mut amounts = amounts.into_iter().map(|a| a.0);
       
        let len = accounts.len();

        if len == 1{
            return Promise::new(accounts.next().unwrap()).create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(amounts.next().unwrap())
        }


        let mut ps: Vec<Promise> = Vec::with_capacity(len);

        //what if we change this to simply dropping new promises
        for (ac, am) in accounts.zip(amounts) {
            ps.push(
                Promise::new(ac)
                    .create_account()
                    .add_full_access_key(env::signer_account_pk())
                    .transfer(am),
            );
        }
        //but what do we return then
        
        ps.into_iter().reduce(|a, b| a.then(b)).unwrap() //btw this then could be 'and' but I'm not sure
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn create_accounts() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MakeWallets::default();
    }
}
