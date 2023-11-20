use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VersionMetadata {
    pub compilation_date_utc: String,
    pub commit_hash: String,
}

impl VersionMetadata {
    pub fn new(compilation_date: &str, commit_hash: &str) -> Self {
        Self {
            compilation_date_utc: compilation_date.to_string(),
            commit_hash: commit_hash.to_string(),
        }
    }
}