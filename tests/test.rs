use near_sdk::AccountId;
use near_sdk::{assert_one_yocto, env, Gas, Promise};
use near_self_update::SelfUpdate;

#[derive(SelfUpdate)]
struct Contract {
    pub manager: AccountId,
}

#[test]
#[should_panic(expected = "Only the manager can update the code")]
fn test_macro() {
    let contract = Contract {
        manager: AccountId::new_unchecked("alice".to_string()),
    };
    contract.update_contract();
}