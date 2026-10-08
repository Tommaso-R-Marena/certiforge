#!/usr/bin/env bash
# Source this file. The digest records the official HTTPS release download;
# this older Lean release does not publish an independent artifact digest.
set -euo pipefail
if command -v lean >/dev/null 2>&1 && lean --version 2>/dev/null | grep -q 'version 4.16.0'; then
  return 0 2>/dev/null || exit 0
fi
test "$(uname -s)" = Linux && test "$(uname -m)" = x86_64
cf_lean_stage="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/certiforge-lean-416.XXXXXX")"
curl --fail --location --retry 3 --output "$cf_lean_stage/lean.tar.zst" \
  https://github.com/leanprover/lean4/releases/download/v4.16.0/lean-4.16.0-linux.tar.zst
echo "cdd31f1064783fff163be53e285722102f16767e65e6fb7f7cb1c232514e0c3a  $cf_lean_stage/lean.tar.zst" | sha256sum --check --status
tar --zstd -xf "$cf_lean_stage/lean.tar.zst" -C "$cf_lean_stage"
rm "$cf_lean_stage/lean.tar.zst"
export PATH="$cf_lean_stage/lean-4.16.0-linux/bin:$PATH"
if [ -n "${GITHUB_PATH:-}" ]; then
  printf '%s\n' "$cf_lean_stage/lean-4.16.0-linux/bin" >> "$GITHUB_PATH"
fi
lean --version | grep 'version 4.16.0'
