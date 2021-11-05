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


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NewAccount {
    account_id: AccountId,
    public_key: PublicKey,
}

impl NewAccount {
    pub fn new(account_id: AccountId, public_key: PublicKey) -> Self { Self { account_id, public_key } }
}

#[ext_contract(ext_self)]
pub trait ExtMakeWallets {
    fn on_account_created(#[callback] val: bool) -> bool;
}

const GAS: u64 = 10_000_000_000_000;

#[near_bindgen]
impl MakeWallets {
    #[payable]
    pub fn make_wallets(new_account: NewAccount) -> Promise {
        Promise::new("near".parse().unwrap()).function_call(
                "create_account".to_string(),
                json!({"new_account_id": new_account.account_id.to_string(), "new_public_key": new_account.public_key}).to_string().as_bytes().to_vec(),
                env::attached_deposit(),
                env::prepaid_gas()/2
           ).then(ext_self::on_account_created(env::current_account_id(), 0, env::prepaid_gas()/3))      
    }

    #[private]
    pub fn on_account_created(#[callback] val: bool) -> bool {
        match val {
            true => env::log_str("account was created successfully"),
            false => env::log_str("error during account creation"),
        }
        val
    }
}
