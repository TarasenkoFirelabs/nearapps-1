use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::{AccountId, PromiseResult, env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearApps {
    any_contracts: bool,
    any_tags: bool,
    approved_contracts: LookupSet<AccountId>,
    required_tags: UnorderedSet<String>,
}

impl Default for NearApps { // todo replace it with initialize function
    fn default() -> Self {
        let mut required_tags = UnorderedSet::new(b"t");
        required_tags.insert(&"person".to_string());
        required_tags.insert(&"organisation".to_string());
        required_tags.insert(&"purpose".to_string());
        Self {
            any_contracts: false,
            any_tags: false,
            approved_contracts: LookupSet::new(b"c"),
            required_tags,
        }
    }
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractArgs {
    function_name: String,
    params: String,
}

impl ContractArgs {
    pub fn new(function_name: String, params: String) -> Self {
        Self {
            function_name,
            params,
        }
    }
}

#[near_bindgen]
impl NearApps {
    pub fn call(&mut self, tags: Vec<HashMap<String, String>>, contract_name: AccountId, args: ContractArgs) {
        self.verify_tags(&tags);
        if self.verify_contract(&contract_name) {
            let p0 = env::promise_create(
                contract_name,
                &args.function_name,
                &args.params.into_bytes(),
                0,
                env::prepaid_gas() / 3,
            );
            env::promise_then(
                p0,
                env::current_account_id(),
                "check_promise",
                json!({ "tags": tags }).to_string().as_bytes(),
                0,
                env::prepaid_gas() / 3,
            );
        }
    }

    fn verify_contract(&self, contract_name: &AccountId) -> bool {
        match self.any_contracts {
            true => true,
            false => self.approved_contracts.contains(contract_name),
        }
    }

    fn verify_tags(&self, tags: &Vec<HashMap<String, String>>) {
        if self.any_tags {
            return;
        } else {
            for str in self.required_tags.iter() {
                for tag in tags.iter() {
                    if !tag.contains_key(&str) {
                        env::panic_str("missing key");
                    }
                }
            }
        }
    }

    pub fn print_required_tags(self) {
        for tag in self.required_tags.iter() {
            env::log_str(&tag);
        }
    }

    #[private]
    pub fn add_contract(&mut self, contract_name: AccountId) {
        self.approved_contracts.insert(&contract_name);
    }

    #[private]
    pub fn remove_contract(&mut self, contract_name: AccountId) {
        self.approved_contracts.remove(&contract_name);
    }

    #[private]
    pub fn any_contracts_allowed(&mut self, any: bool) {
        self.any_contracts = any;
    }

    #[private]
    pub fn add_tag(&mut self, tag_name: String) {
        self.required_tags.insert(&tag_name);
    }

    #[private]
    pub fn remove_tag(&mut self, tag_name: String) {
        self.required_tags.remove(&tag_name);
    }

    #[private]
    pub fn any_tags_allowed(&mut self, any: bool) {
        self.any_tags = any;
    }

    #[private]
    pub fn check_promise(&mut self, tags: Vec<HashMap<String, String>>) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                if tags.len() > 0{
                    env::log_str(&serde_json::to_string(&tags).unwrap());
                }
            }
            _ => env::panic_str("Promise with index 0 failed"),
        };
    }
}
