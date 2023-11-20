mod tests;

use model::api::ContractApi;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseOrValue,
};
use near_self_update_model::VersionMetadata;

const COMPILATION_DATETIME: &str = env!("COMPILATION_DATETIME");
const COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// The account ID authorized to perform sensitive operations on the contract.
    pub manager: AccountId,

    release_notes: String,
}

#[near_bindgen]
impl ContractApi for Contract {
    #[init]
    #[private]
    fn init(manager: AccountId) -> Self {
        Self {
            manager,
            release_notes: "Initial contract state".to_string(),
        }
    }

    fn version_metadata(&self) -> VersionMetadata {
        VersionMetadata::new(COMPILATION_DATETIME, COMMIT_HASH, &self.release_notes)
    }

    fn update_contract(&mut self, release_notes: String) -> PromiseOrValue<()> {
        assert_ne!(self.release_notes, release_notes, "Please update release notes");

        assert_eq!(
            env::predecessor_account_id(),
            self.manager,
            "Only the manager can update the contract"
        );

        let code = env::input().expect("Error: No input").to_vec();

        Promise::new(env::current_account_id())
            .deploy_contract(code)
            // .function_call("after_update".to_string(), vec![], 0, Gas(200_000_000_000_000))
            .as_return()
            .into()
    }

    fn after_update(&mut self) {
        log!("Hellooo");
        log!(format!("{:?}", self.version_metadata()));
    }
}
