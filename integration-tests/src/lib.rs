mod context;

use std::{env, fs};

use anyhow::Result;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn update() -> Result<()> {
    let mut context = prepare_contract().await?;

    let manager = context.manager().await?;

    let code = load_wasm("../res/updated_contract.wasm");

    context
        .test_contract()
        .update_contract(code)
        .with_user(&manager)
        .await?;

    Ok(())
}

fn load_wasm(wasm_path: &str) -> Vec<u8> {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path)).expect("Failed to get wasm file path");
    fs::read(wasm_filepath).expect("Failed to load wasm")
}
