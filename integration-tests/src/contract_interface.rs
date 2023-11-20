use anyhow::Result;
use async_trait::async_trait;
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;
use near_self_update_model::VersionMetadata;
use near_workspaces::{Account, Contract};

pub const TEST_CONTRACT: &str = "test_contract";

pub struct TestContract<'a> {
    contract: &'a Contract,
    account: Option<Account>,
}

#[async_trait]
impl ContractApiIntegration for TestContract<'_> {
    async fn init(&self) -> Result<()> {
        self.call_contract("init", ()).await
    }

    async fn version_metadata(&self) -> Result<VersionMetadata> {
        self.call_contract("version_metadata", ()).await
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
            .expect("Set account with `user` method first")
            .clone()
    }

    fn contract(&self) -> &'a Contract {
        self.contract
    }
}
