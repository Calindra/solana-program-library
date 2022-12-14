#!/bin/bash

set -e
cargo build
cp ../../target/debug/spl-associated-token-account ../../../rollups-examples/solana-adapter/solana_smart_contract_bin/ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
