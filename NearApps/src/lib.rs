use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::{env, near_bindgen, AccountId, PromiseResult};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearApps {
    any_contracts: bool,
    any_tags: bool,
    owner_id: AccountId,
    approved_contracts: LookupSet<AccountId>,
    required_tags: UnorderedSet<String>,
}

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner(),
            "Ownable: predecessor is not the owner"
        );
    }
    fn owner(&self) -> AccountId;
    fn transfer_ownership(&mut self, owner: AccountId);
}

#[near_bindgen]
impl Ownable for NearApps {
    fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

impl Default for NearApps {
    fn default() -> Self {
        let mut required_tags = UnorderedSet::new(b"t");
        required_tags.insert(&"person".to_string());
        required_tags.insert(&"organisation".to_string());
        required_tags.insert(&"purpose".to_string());
        Self {
            any_contracts: false,
            any_tags: false,
            owner_id: env::current_account_id(),
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
    #[init]
    pub fn new(owner_id: AccountId, tags: Vec<String>) -> Self {
        let mut required_tags = UnorderedSet::new(b"t");
        required_tags.extend(tags);
        Self {
            any_contracts: false,
            any_tags: false,
            owner_id,
            approved_contracts: LookupSet::new(b"c"),
            required_tags,
        }
    }

    pub fn call(
        &mut self,
        tags: Vec<HashMap<String, String>>,
        contract_name: AccountId,
        args: ContractArgs,
    ) {
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
        if !self.any_tags {
            if tags.len() == 0 {
                env::panic_str("empty tags");
            }
            for str in self.required_tags.iter() {
                for tag in tags {
                    if !tag.contains_key(&str) {
                        env::panic_str("missing key");
                    }
                }
            }
        }
    }

    pub fn print_required_tags(self) {
        let s = format!("{:?}", self.required_tags.iter().collect::<Vec<String>>());
        env::log_str(&s[1..s.len()]);
    }

    pub fn add_contract(&mut self, contract_name: AccountId) {
        self.assert_owner();
        self.approved_contracts.insert(&contract_name);
    }

    pub fn remove_contract(&mut self, contract_name: AccountId) {
        self.assert_owner();
        self.approved_contracts.remove(&contract_name);
    }

    pub fn any_contracts_allowed(&mut self, any: bool) {
        self.assert_owner();
        self.any_contracts = any;
    }

    pub fn add_tag(&mut self, tag_name: String) {
        self.assert_owner();
        self.required_tags.insert(&tag_name);
    }

    pub fn remove_tag(&mut self, tag_name: String) {
        self.assert_owner();
        self.required_tags.remove(&tag_name);
    }

    pub fn any_tags_allowed(&mut self, any: bool) {
        self.assert_owner();
        self.any_tags = any;
    }

    #[private]
    pub fn check_promise(&mut self, tags: Vec<HashMap<String, String>>) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                if tags.len() > 0 {
                    env::log_str(&serde_json::to_string(&tags).unwrap());
                }
            }
            _ => env::panic_str("Promise with index 0 failed"),
        };
    }
}
