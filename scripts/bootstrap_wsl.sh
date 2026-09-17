#!/usr/bin/env bash
# Bootstrap CERTIFORGE on Linux / WSL2 (idempotent).
set -euo pipefail

echo "==> Rust"
if ! command -v rustc >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
rustup default stable
rustc --version

echo "==> elan / Lean"
if ! command -v elan >/dev/null 2>&1; then
  curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh -s -- -y --default-toolchain none
  export PATH="$HOME/.elan/bin:$PATH"
fi
export PATH="$HOME/.elan/bin:${PATH:-}"
cd "$(dirname "$0")/../formal"
elan toolchain install "$(cat lean-toolchain)"
lake build

echo "==> Rust workspace"
cd "$(dirname "$0")/.."
cargo build --workspace
cargo test --workspace

echo "==> Bootstrap complete"
