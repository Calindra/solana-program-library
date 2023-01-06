#!/bin/bash

set -e
cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release

cp ./target/riscv64ima-cartesi-linux-gnu/release/spl-token ../../../rollups-examples/solana-adapter/solana_smart_contract_bin/TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

cargo clean
echo "done."
