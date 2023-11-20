use async_trait::async_trait;
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;
use near_workspaces::Account;

use crate::contract_interface::{TestContract, TEST_CONTRACT};

pub type Context = integration_utils::context::Context<near_workspaces::network::Sandbox>;

#[async_trait]
pub trait IntegrationContext {
    async fn manager(&mut self) -> anyhow::Result<Account>;
    async fn alice(&mut self) -> anyhow::Result<Account>;
    async fn fee(&mut self) -> anyhow::Result<Account>;
    fn test_contract(&self) -> TestContract;
}

#[async_trait]
impl IntegrationContext for Context {
    async fn manager(&mut self) -> anyhow::Result<Account> {
        self.account("manager").await
    }

    async fn alice(&mut self) -> anyhow::Result<Account> {
        self.account("alice").await
    }

    async fn fee(&mut self) -> anyhow::Result<Account> {
        self.account("fee").await
    }

    fn test_contract(&self) -> TestContract {
        TestContract::with_contract(&self.contracts[TEST_CONTRACT])
    }
}

pub(crate) async fn prepare_contract() -> anyhow::Result<Context> {
    let context = Context::new(&[TEST_CONTRACT], "build-integration".into()).await?;
    context.test_contract().init().await?;
    Ok(context)
}
