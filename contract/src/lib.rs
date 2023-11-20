mod tests;

use model::api::ContractApi;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};
use near_self_update_model::VersionMetadata;

const COMPILATION_DATETIME: &str = env!("COMPILATION_DATETIME");
const COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {}

#[near_bindgen]
impl ContractApi for Contract {
    #[init]
    #[private]
    fn init() -> Self {
        Self {}
    }

    fn version_metadata(&self) -> VersionMetadata {
        VersionMetadata::new(COMPILATION_DATETIME, COMMIT_HASH)
    }
}
