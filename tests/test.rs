use near_sdk::AccountId;
use near_sdk::{env, Gas, Promise};
use near_self_update::SelfUpdate;

#[derive(SelfUpdate)]
struct Contract {
    pub manager: AccountId,
}

#[test]
fn test_macro() {
    let contract = Contract {
        manager: AccountId::new_unchecked("alice".to_string()),
    };
    contract.update_contract();
}