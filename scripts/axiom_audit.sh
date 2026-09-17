#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="$HOME/.elan/bin:${PATH:-}"
cd "$ROOT/formal"
lake env lean "$ROOT/formal/CertiForge/AxiomAudit.lean"
