use std::str;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen};

extern crate base64;
use base64::{decode};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Call {}
#[near_bindgen]
impl Call {
    pub fn log_analytics(encoded: String) {
        let call_encoded: Vec<&str> = encoded.split('_').collect();
        let app_id_encoded = call_encoded[0];
        let app_id = str::from_utf8(&decode(app_id_encoded).unwrap()[..])
            .unwrap()
            .to_string();
        let action_id_encoded = call_encoded[1];
        let action_id = str::from_utf8(&decode(action_id_encoded).unwrap()[..])
            .unwrap()
            .to_string();
        let user_name_encoded = call_encoded[2];
        let user_name = str::from_utf8(&decode(user_name_encoded).unwrap()[..])
            .unwrap()
            .to_string();

        env::log_str(&format!(
            "app_id: {}, action_id: {}, user_name: {}",
            app_id, action_id, user_name
        ));
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
        let action_id = "actionid".to_string();
        let user_name = "user_123".to_string();
        let api_call = Call {
            app_id,
            action_id,
            user_name,
        };
        let mut call_decoded = Call::default();
        call_decoded.log_analytics(api_call.generate_string());

        assert_eq!(api_call.app_id, call_decoded.app_id);
        assert_eq!(api_call.action_id, call_decoded.action_id);
        assert_eq!(api_call.user_name, call_decoded.user_name);
    }
    #[test]
    fn encoding_2() {
        let app_id = "Somestring_12345UVW".to_string();
        let action_id = "Someaccount_ABC6789".to_string();
        let user_name = "123_Someuser".to_string();
        let api_call = Call {
            app_id,
            action_id,
            user_name,
        };
        let mut call_decoded = Call::default();
        call_decoded.log_analytics(api_call.generate_string());

        assert_eq!(api_call.app_id, call_decoded.app_id);
        assert_eq!(api_call.action_id, call_decoded.action_id);
        assert_eq!(api_call.user_name, call_decoded.user_name);
    }
    #[test]
    fn encoding_3() {
        let api_call = Call::default();
        let call_decoded = Call::default();
        assert_eq!(api_call.app_id, call_decoded.app_id);
        assert_eq!(api_call.action_id, call_decoded.action_id);
        assert_eq!(api_call.user_name, call_decoded.user_name);
    }
}