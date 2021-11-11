use near_contract_standards::non_fungible_token::TokenId;
use crate::*;
use near_sdk::{AccountId};
use near_sdk::serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropRewards(pub Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropReward {
    pub account_id: AccountId,
    pub token_id: TokenId,
}

pub trait SupportsAirdrop {
    fn airdrop(&mut self, rewards: AirdropRewards);
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>);
    fn pending_rewards_by_key(&self, account: &AccountId) -> TokenId;
}

#[near_bindgen]
impl SupportsAirdrop for NftContract {
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>) {
        self.assert_owner();
        for reward in rewards {
            let account_id = reward.0.to_string();
            let token_id = reward.1;
            self.pending_nft_rewards.insert(&account_id, &token_id);
        }
    }

    fn airdrop(&mut self, rewards: AirdropRewards) {
        self.assert_owner();
        for reward in rewards.0 {
            let account = reward.account_id.to_string();
            self.nft_transfer_unsafe(&reward.token_id, &self.owner(), &account);
        }
    }

    fn pending_rewards_by_key(&self, account: &AccountId) -> TokenId {
        self.pending_nft_rewards.get(&account).unwrap()
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

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
        //let valid_account = TryFrom::try_from("test_airdop_owner.testnet").unwrap();
        let account_id:AccountId="test_airdop_owner.testnet".into();
        let mut contract = NftContract::new_default_meta(account_id);
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.testnet".to_string(),
            token_id: "token".to_string(),
        };
        let token_id=reward.token_id.clone();
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![("test_airdop_owner.testnet".to_string(), token_id)]);
        contract.airdrop(rewards);
    }

    #[test]
    fn test_airdrop_default_meta() {
        let mut context = VMContextBuilder::new();
        let valid_owner_id=TryFrom::try_from("test_airdop_owner.testnet").unwrap();
        let testnet_account_id:AccountId="test_airdop_owner.testnet".into();

        context.predecessor_account_id(valid_owner_id);
        testing_env!(context.build());

        //let valid_owner: ValidAccountId = TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap();
        let owner = "test_airdop_owner.testnet".to_string();
        //let valid_owner: ValidAccountId = TryFrom::try_from("test_airdop_owner.testnet".to_string()).unwrap();
        let mut contract = NftContract::new_default_meta(testnet_account_id);
        let mut contract = NftContract::new_default_meta(
            TryFrom::try_from(owner.clone()).unwrap(),
        );
        println!(
            "{}",
            &contract
                .token
                .tokens_per_owner
                .as_ref()
                .unwrap()
                .contains_key(&owner)
        );
        near_sdk::env::attached_deposit() 
        let token_meta = TokenMetadata{
            title: Some("TestMetadata".to_string()),
            description: None, 
            media: None, 
            media_hash: None, ;./,;'
            copies: None, 
            issued_at: None, 
            expires_at: None, 
            starts_at: None, 
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None, 
        };
        contract.nft_mint("New_test_token".to_string(), TryFrom::try_from(owner.clone()).unwrap(), token_meta);
        println!(
            "{}",
            &contract
                .token
                .tokens_per_owner
                .as_ref()
                .unwrap()
                .contains_key(&owner)
        );
        let token_id = (&contract.token.tokens_per_owner.as_ref().unwrap())
            .get(&owner)
            .unwrap()
            .to_vec()[0]
            .clone();
        let reward = AirdropReward {
            account_id: "test_airdop_receiver.testnet".to_string(),
            token_id: token_id.clone(),
        };
        let rewards = AirdropRewards(vec![reward]);
        contract.add_pending_rewards(vec![(owner, token_id)]);
        contract.airdrop(rewards);
    }
}  

mod tests{
    use super::*;
    use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount, to_yocto};

    extern crate app_nft;
    use app_nft::NftContractContract;

    near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
        AIRDROP_BYTES => "../target/wasm32-unknown-unknown/release/app-nft.wasm",
    }

    const CONTRACT_ID: &str = "contract";

    pub fn init() -> (UserAccount, ContractAccount<NftContractContract>, UserAccount) {
        // Use `None` for default genesis configuration; more info below
        let root = init_simulator(None);
    
        let contract = deploy!(
            contract: NftContractContract,
            contract_id: CONTRACT_ID,
            bytes: &AIRDROP_BYTES,
            signer_account: root
        );
    
        let alice = root.create_user(
            "alice".parse().unwrap(),
            to_yocto("100") // initial balance
        );
    
        (root, contract, alice)
    }

    //#[test]
    //#[should_panic(expected = "Ownable: predecessor is not the owner")]
    fn simulate_airdrop_default_meta() {
        let (root, contract, alice) = init();

        let owner = root.account_id;
        let res = call!(
            root,
            NftContractContract::new_default_meta(TryFrom::try_from(owner.clone()).unwrap()),
        ).unwrap_json();

        let token_meta = TokenMetadata{
            title: Some("TestMetadata".to_string()),
            description: None, 
            media: None, 
            media_hash: None,
            copies: None, 
            issued_at: None, 
            expires_at: None, 
            starts_at: None,
            updated_at: None, 
            extra: None, 
            reference: None, 
            reference_hash: None, 
        };

        call! {
            contract.nft_mint("New_test_token".to_string(), TryFrom::try_from(owner.clone()).unwrap(), token_meta),
            gas = DEFAULT_GAS
        }; 
    } 
} */
    


