#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="$HOME/.cargo/bin:/usr/local/cargo/bin:$HOME/.elan/bin:${PATH:-}"
cd "$ROOT"
cargo test --workspace
(cd formal && lake build)
cf_test_stage="$(mktemp -d "${TMPDIR:-/tmp}/certiforge-package-test.XXXXXX")"
trap 'rm -rf "$cf_test_stage"' EXIT
cargo run -q -p certiforge-cli -- package build benchmarks/bitvector/or_via_add.certir --out "$cf_test_stage/or_via_add" --seed 1
cargo run -q -p certiforge-cli -- package verify "$cf_test_stage/or_via_add"
cargo run -q -p certiforge-cli -- attack "$cf_test_stage/or_via_add"
echo "ALL OK"
