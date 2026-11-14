#!/usr/bin/env bash
# Builds the Bevy sim (sim/) to WASM into public/sim/. Outputs are committed: Vercel has no Rust toolchain.
# Needs: rustup target wasm32-unknown-unknown, wasm-bindgen-cli 0.2.130 (see docs / plan Step 0).
set -euo pipefail
cd "$(dirname "$0")/.."
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"

cargo build --manifest-path sim/Cargo.toml --lib --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen ../mars_sim_target/wasm32-unknown-unknown/wasm-release/mars_sim.wasm \
  --target web --no-typescript --out-dir public/sim --out-name mars_sim
if ! npx -y -p binaryen wasm-opt -Oz public/sim/mars_sim_bg.wasm -o public/sim/mars_sim_bg.wasm; then
  echo "warning: wasm-opt failed; keeping the unoptimized wasm" >&2
fi
du -h public/sim/*
