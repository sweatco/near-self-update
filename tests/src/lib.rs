#[cfg(test)]
mod tests {
    use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, Gas, Promise};
    use near_self_update::SelfUpdate;

    #[derive(SelfUpdate)]
    #[near_bindgen]
    pub struct Contract {
        pub manager: AccountId,
    }

    #[test]
    #[should_panic(expected = "Only the manager can update the code")]
    fn test_macro() {
        let mut contract = Contract {
            manager: AccountId::new_unchecked("alice".to_string()),
        };
        contract.update_contract();
    }
}
