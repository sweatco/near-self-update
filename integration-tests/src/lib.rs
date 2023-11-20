mod context;
mod contract_interface;

use anyhow::Result;
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn update() -> Result<()> {
    let mut context = prepare_contract().await?;

    let manager = context.manager().await?;

    let version = context.test_contract().version_metadata().await?;

    assert_eq!(version.release_notes, "Initial contract state");

    println!("{version:?}");

    context
        .test_contract()
        .with_user(&manager)
        .update_contract("Added some stuff to contract and fixed some bugs".as_bytes())
        .await?;

    let version = context.test_contract().version_metadata().await?;

    assert_eq!(
        version.release_notes,
        "Added some stuff to contract and fixed some bugs"
    );

    println!("{version:?}");

    Ok(())
}
