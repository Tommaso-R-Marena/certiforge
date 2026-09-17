#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="$HOME/.cargo/bin:/usr/local/cargo/bin:$HOME/.elan/bin:${PATH:-}"
cd "$ROOT"
cargo test --workspace
(cd formal && lake build)
cargo run -q -p certiforge-cli -- package build benchmarks/bitvector/or_via_add.certir --out artifacts/or_via_add --seed 1
cargo run -q -p certiforge-cli -- package verify artifacts/or_via_add
cargo run -q -p certiforge-cli -- attack artifacts/or_via_add
echo "ALL OK"
