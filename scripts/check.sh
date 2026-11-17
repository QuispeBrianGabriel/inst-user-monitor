#!/usr/bin/env bash
# Comprobaciones de calidad. Es exactamente lo que ejecuta el pipeline de CI.
set -euo pipefail
cd "$(dirname "$0")/.."

step() { printf "\n\033[1;33m▸ %s\033[0m\n" "$1"; }

step "Formato (rustfmt)"
cargo fmt --check

step "Formato (leptosfmt, macros view!)"
leptosfmt --check src

step "Lint (clippy, wasm32)"
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings

step "Tests"
cargo test

step "Build (trunk)"
trunk build

printf "\n\033[1;32m✔ Todo correcto\033[0m\n"
