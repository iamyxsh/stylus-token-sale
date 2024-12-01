#!/bin/bash

# Build wasm32-unknown-unknown binary
# cargo build --locked --release --target wasm32-unknown-unknown
cargo stylus check

# Run tests
export RPC_URL=http://localhost:8547
cargo watch -x 'test --locked --test "integration_tests" --features export-abi -- --nocapture'