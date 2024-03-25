mod tests;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseOrValue,
};
use near_self_update_model::{SelfUpdateApi, SelfUpdateCallback, VersionMetadata};
use update_model::api::ContractApi;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// The account ID authorized to perform sensitive operations on the contract.
    pub manager: AccountId,
}

#[near_bindgen]
impl ContractApi for Contract {
    #[init]
    #[private]
    fn init(manager: AccountId) -> Self {
        Self { manager }
    }
}

#[near_bindgen]
impl SelfUpdateApi for Contract {
    fn version_metadata(&self) -> VersionMetadata {
        VersionMetadata::new(
            env!("COMPILATION_DATETIME"),
            env!("GIT_COMMIT_HASH"),
            env!("CONTRACT_RELEASE_NOTES"),
        )
    }

    fn update_contract(&mut self) -> PromiseOrValue<()> {
        assert_eq!(
            env::predecessor_account_id(),
            self.manager,
            "Only the manager can update the contract"
        );

        let code = env::input().expect("Error: No input");

        log!("update_contract");

        Promise::new(env::current_account_id())
            .deploy_contract(code)
            .function_call("after_update".to_string(), vec![], 0, Gas(200_000_000_000_000))
            .as_return()
            .into()
    }
}

#[near_bindgen]
impl SelfUpdateCallback for Contract {
    fn after_update(&mut self) {
        log!("after_update");
        log!(format!("{:?}", self.version_metadata()));
    }
}
