#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo run -p hw_tools -- validate
cargo run -p hw_tools -- sim-replay --blueprint examples/blueprints/starter_rescue_skiff.json --seconds 10
