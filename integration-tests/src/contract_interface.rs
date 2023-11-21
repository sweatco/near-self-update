use anyhow::Result;
use async_trait::async_trait;
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;
use near_sdk::{serde_json::json, AccountId};
use near_self_update_model::{SelfUpdateApiIntegration, VersionMetadata};
use near_workspaces::{Account, Contract};

pub const TEST_CONTRACT: &str = "test_contract";

pub struct TestContract<'a> {
    contract: &'a Contract,
    account: Option<Account>,
}

#[async_trait]
impl ContractApiIntegration for TestContract<'_> {
    async fn init(&self, manager: AccountId) -> Result<()> {
        self.call_contract("init", json!({ "manager": manager })).await
    }
}

#[async_trait]
impl SelfUpdateApiIntegration for TestContract<'_> {
    async fn version_metadata(&self) -> Result<VersionMetadata> {
        self.call_contract("version_metadata", ()).await
    }

    async fn update_contract(&mut self, code: Vec<u8>) -> Result<()> {
        println!("▶️ update_contract");

        let transaction = self.user_account().call(self.contract().id(), "update_contract");

        let result = transaction.args(code).max_gas().transact().await?.into_result()?;

        println!("Result: {:?}", result);

        Ok(())
    }

    async fn after_update(&mut self) -> Result<()> {
        self.call_contract("after_update", ()).await
    }
}

impl<'a> IntegrationContract<'a> for TestContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self {
        Self {
            contract,
            account: None,
        }
    }

    fn with_user(mut self, account: &Account) -> Self {
        self.account = account.clone().into();
        self
    }

    fn user_account(&self) -> Account {
        self.account
            .as_ref()
            .expect("Set account with `with_user` method first")
            .clone()
    }

    fn contract(&self) -> &'a Contract {
        self.contract
    }
}
