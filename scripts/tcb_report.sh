#!/usr/bin/env bash
# Rough TCB vs untrusted LOC metrics (not gamed: lists what we count).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
trusted=$(find "$ROOT/formal/CertiForge" "$ROOT/crates/certiforge-package/src" "$ROOT/crates/certir/src" "$ROOT/crates/certir-parser/src" "$ROOT/crates/certir-interpreter/src" -name '*.lean' -o -name '*.rs' | xargs wc -l | tail -1 | awk '{print $1}')
untrusted=$(find "$ROOT/crates/forgeopt" "$ROOT/crates/forgeopt-search" "$ROOT/crates/forgeopt-cost" "$ROOT/crates/forgeopt-bench" "$ROOT/crates/certiforge-mutate" "$ROOT/crates/certiforge-fuzz" -name '*.rs' | xargs wc -l | tail -1 | awk '{print $1}')
ratio=$(python3 -c "print(round($untrusted / max($trusted,1), 3))")
mkdir -p "$ROOT/results"
python3 - <<PY
import json
print(json.dumps({
  "trusted_loc_approx": $trusted,
  "untrusted_automation_loc_approx": $untrusted,
  "ratio_untrusted_over_trusted": $ratio,
  "notes": "Trusted count includes Lean formal + Rust certir+package verifier. Untrusted includes ForgeOpt/search/bench/mutate/fuzz. Lean kernel and Rustc not included in LOC."
}, indent=2))
PY
python3 - <<PY > "$ROOT/results/tcb_metrics.json"
import json
json.dump({
  "trusted_loc_approx": $trusted,
  "untrusted_automation_loc_approx": $untrusted,
  "ratio_untrusted_over_trusted": $ratio,
  "notes": "Trusted count includes Lean formal + Rust certir+package verifier. Untrusted includes ForgeOpt/search/bench/mutate/fuzz. Lean kernel and Rustc not included in LOC."
}, open("$ROOT/results/tcb_metrics.json","w"), indent=2)
print("wrote results/tcb_metrics.json")
PY
