#!/bin/sh
# Build the WASM core for the demo page, then run the tests.
set -eu
cd "$(dirname "$0")"
[ -f Cargo.lock ] || cargo generate-lockfile --offline
cargo test --locked
cargo build --locked --release --lib --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/gtm_10_auto_qbr.wasm web/app.wasm
ls -l web/app.wasm
