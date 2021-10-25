use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use std::{fmt, str};

extern crate base64;

use base64::{encode, decode};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Call {
    pub app_id: String,
    pub action_id: String,
    pub user: String,
}

#[near_bindgen]
impl Call {
    #[init]
    pub fn new() -> Self {
	Self {
	    app_id: String::new(),
	    action_id: String::new(),
	    user: String::new(),
	}    
    }

    pub fn generate_string(&self) -> String {
    	format!("{}_{}_{}", encode(self.app_id.clone()), encode(self.action_id.clone()), encode(self.user.clone()))	
    }

    pub fn blockchain_analytics(encoded: String) -> Self {
	let call_encoded: Vec<&str> = encoded.split('_').collect();
	let app_id_encoded = call_encoded[0];
	let app_id = str::from_utf8(&decode(app_id_encoded).unwrap()[..]).unwrap().to_string();
	let action_id_encoded = call_encoded[1];
        let action_id = str::from_utf8(&decode(action_id_encoded).unwrap()[..]).unwrap().to_string();
	let user_encoded = call_encoded[2];
        let user = str::from_utf8(&decode(user_encoded).unwrap()[..]).unwrap().to_string();
	env::log_str("this is a log");
	Call {app_id, action_id, user}	
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn encoding_1() {
	let app_id = "appid".to_string();
	let action_id =  "actionid".to_string();
	let user = "user123".to_string();
	let api_call = Call{app_id, action_id, user};
	let call_decoded = Call::blockchain_analytics(api_call.generate_string());
	assert_eq!(api_call.app_id, call_decoded.app_id);
	assert_eq!(api_call.action_id, call_decoded.action_id);
	assert_eq!(api_call.user, call_decoded.user);
    }

    #[test]
    fn encoding_2() {
        let app_id = "Somestring12345UVW".to_string();
        let action_id =  "SomeaccountABC6789".to_string();
        let user = "123Someuser".to_string();
        let api_call = Call{app_id, action_id, user};
        let call_decoded = Call::blockchain_analytics(api_call.generate_string());
        assert_eq!(api_call.app_id, call_decoded.app_id);
        assert_eq!(api_call.action_id, call_decoded.action_id);
        assert_eq!(api_call.user, call_decoded.user);
    }

    #[test]
    fn encoding_3() {
        let api_call = Call::new();
        let call_decoded = Call::blockchain_analytics(api_call.generate_string());
        assert_eq!(api_call.app_id, call_decoded.app_id);
        assert_eq!(api_call.action_id, call_decoded.action_id);
        assert_eq!(api_call.user, call_decoded.user);
    }

}
