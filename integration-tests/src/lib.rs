mod context;
mod contract_interface;

use std::{env, fs};

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

    let code = load_wasm("../res/updated_contract.wasm");

    dbg!(code.len());

    context
        .test_contract()
        .with_user(&manager)
        .update_contract(code)
        .await?;

    let version = context.test_contract().version_metadata().await?;

    assert_eq!(version.release_notes, "Updated contract with some stuff");

    println!("{version:?}");

    Ok(())
}

fn load_wasm(wasm_path: &str) -> Vec<u8> {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path)).expect("Failed to get wasm file path");
    fs::read(wasm_filepath).expect("Failed to load wasm")
}
