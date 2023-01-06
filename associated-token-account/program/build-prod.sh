#!/bin/bash

set -e
cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release

cp ./target/riscv64ima-cartesi-linux-gnu/release/spl-associated-token-account ../../../rollups-examples/solana-adapter/solana_programs_riscv/ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL

cargo clean
echo "done."
