#[cfg(test)]
mod tests {
    use near_sdk::{env, near_bindgen, require, AccountId};
    use near_self_update::SelfUpdate;

    #[derive(SelfUpdate)]
    #[near_bindgen]
    pub struct Contract {
        pub manager: AccountId,
    }

    impl Contract {
        fn assert_update(&self) {
            require!(
                self.manager == env::predecessor_account_id(),
                "Only the manager can update the code"
            );
        }
    }

    #[test]
    #[should_panic(expected = "Only the manager can update the code")]
    fn test_macro() {
        let mut contract = Contract {
            manager: AccountId::new_unchecked("alice".to_string()),
        };

        contract.update_contract(vec![], None);
    }
}
