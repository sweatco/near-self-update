use integration_trait::make_integration_version;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    PromiseOrValue,
};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VersionMetadata {
    pub compilation_date_utc: String,
    pub commit_hash: String,
    pub release_notes: String,
}

impl VersionMetadata {
    pub fn new(compilation_date: &str, commit_hash: &str, release_notes: &str) -> Self {
        Self {
            compilation_date_utc: compilation_date.to_string(),
            commit_hash: commit_hash.to_string(),
            release_notes: release_notes.to_string(),
        }
    }
}

pub trait SelfUpdateCallback {
    fn after_update(&mut self);
}

#[make_integration_version]
pub trait SelfUpdateApi: SelfUpdateCallback {
    fn version_metadata(&self) -> VersionMetadata;
    #[update]
    fn update_contract(&mut self) -> PromiseOrValue<()>;
}
