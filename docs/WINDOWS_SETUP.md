# Windows Setup

Primary development in Phase I is Linux (cloud VM / WSL2). Windows native is supported for
the Rust CLI; Lean is most reliable under WSL2.

## Recommended: WSL2

1. Install WSL2 + Ubuntu.
2. Inside WSL, run `./scripts/bootstrap_wsl.sh`.
3. Use `cargo` / `lake` from the WSL tree (same repo checkout).

## Native Windows (Rust only)

1. Install Rustup (stable).
2. `cargo build --workspace`
3. Lean: install elan for Windows or use WSL for `formal/`.

Do not require Docker.
