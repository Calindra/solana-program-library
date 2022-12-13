#!/bin/bash

set -e
cargo build
cp ../../target/debug/spl-token ../../../rollups-examples/solana-adapter/solana_smart_contract_bin/TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
