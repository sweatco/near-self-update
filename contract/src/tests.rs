#![cfg(test)]

use model::api::ContractApi;
use near_sdk::AccountId;

use crate::Contract;

#[test]
fn test_version_metadata() {
    let manager = AccountId::new_unchecked("manager".to_string());
    let contract = Contract::init(manager);
    let version_metadata = contract.version_metadata();
    println!("{:?}", contract.version_metadata());
    assert_eq!(version_metadata.release_notes, "Initial contract state");
}
