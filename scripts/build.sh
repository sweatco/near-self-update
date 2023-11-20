#!/bin/bash
set -eox pipefail

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build -p test-contract --target wasm32-unknown-unknown --profile=contract

cp ./target/wasm32-unknown-unknown/contract/test_contract.wasm res/test_contract.wasm
