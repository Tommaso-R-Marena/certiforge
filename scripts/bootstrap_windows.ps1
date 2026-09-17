# Bootstrap CERTIFORGE on Windows (Rust-focused). Prefer WSL for Lean.
$ErrorActionPreference = "Stop"

Write-Host "==> Checking rustc"
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
  Write-Host "Install Rust from https://rustup.rs then re-run."
  exit 1
}
rustc --version
cargo build --workspace
cargo test --workspace

Write-Host "Lean: use WSL2 and scripts/bootstrap_wsl.sh for formal/ builds."
Write-Host "Bootstrap (Windows Rust) complete."
