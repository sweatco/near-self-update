#![cfg(test)]

use model::api::ContractApi;

use crate::Contract;

#[test]
fn test_version_metadata() {
    let contract = Contract::init();
    println!("{:?}", contract.version_metadata());
}
