mod context;
mod contract_interface;

use anyhow::Result;
use model::api::ContractApiIntegration;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn update() -> Result<()> {
    let context = prepare_contract().await?;

    println!("{:?}", context.test_contract().version_metadata().await?);

    Ok(())
}
