use crate::*;

pub(crate) fn assert_self() {
    assert_eq!(
        env::predecessor_account_id(),
        env::current_account_id(),
        "Method is private"
    );
}

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}
pub(crate) fn promise_is_succeeded() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}

#[near_bindgen]
impl Ownable for NftContract {
    fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}
