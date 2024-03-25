use near_workspaces::Account;
use nitka::misc::ToNear;
use update_model::api::{ContractApiIntegration, UpdateContract};

pub type Context = nitka::context::Context<near_workspaces::network::Sandbox>;

pub trait IntegrationContext {
    async fn manager(&mut self) -> anyhow::Result<Account>;
    async fn alice(&mut self) -> anyhow::Result<Account>;
    async fn fee(&mut self) -> anyhow::Result<Account>;
    fn test_contract(&self) -> UpdateContract;
}

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

    fn test_contract(&self) -> UpdateContract {
        UpdateContract {
            contract: &self.contracts["update_contract"],
        }
    }
}

pub(crate) async fn prepare_contract() -> anyhow::Result<Context> {
    let mut context = Context::new(&["update_contract"], true, "build-integration".into()).await?;

    let manager = context.manager().await?;

    context.test_contract().init(manager.to_near()).await?;
    Ok(context)
}
