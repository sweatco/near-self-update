use near_workspaces::types::NearToken;
use nitka::{json, ContractCall};

pub trait HasContract {
    fn contract(&self) -> &near_workspaces::Contract;
}

pub trait UpdateApiIntegration: HasContract {
    fn update_contract(&self, code: Vec<u8>, callback: Option<String>) -> ContractCall<()> {
        ContractCall::new("update_contract", self.contract().clone())
            .args_json(json!({
                "code": code,
                "callback": callback
            }))
            .unwrap()
            .deposit(NearToken::from_yoctonear(1))
    }

    fn contract_version(&self) -> ContractCall<String> {
        ContractCall::new("contract_version", self.contract().clone())
    }
}

impl<T: HasContract> UpdateApiIntegration for T {}
