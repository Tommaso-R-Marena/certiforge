# CERTIFORGE

**Proof-Carrying AI Software with Proof-Preserving Superoptimization**

CERTIFORGE packages untrusted AI-generated (or otherwise untrusted) programs together
with independently checkable evidence that they satisfy a formal specification and that
aggressively optimized variants remain semantically equivalent.

> The generating intelligence is not the security boundary.  
> The optimizer is not the security boundary.  
> The independently checkable artifact is the security boundary.

## Phase I status

Phase I delivers a closed pure core for straight-line CertIR bitvector programs:

- CertIR AST, parser, type checker, Rust interpreter
- Lean 4 canonical semantics + soundness scaffolding
- `bv_decide`-backed bitvector identity proofs (SAT solver untrusted; LRAT checked in Lean)
- Deterministic certificate packages (`certiforge package build|verify`)
- ForgeOpt v1: untrusted rewrite/stochastic search admitted only after equivalence checking
- Adversarial attack suite with **0 false accepts** on the current campaign (7 attacks)

See `docs/PROJECT_STATE.md` and `results/PHASE_I_VERDICT.md` for honest claims and gaps.

## Quick start (Linux / WSL / this cloud VM)

```bash
# Toolchains: Rust stable, elan/Lean 4.16 (see scripts/)
./scripts/bootstrap_wsl.sh

cargo build --workspace
cargo test --workspace
(cd formal && lake build)

# Optimize + package + verify
cargo run -p certiforge-cli -- optimize benchmarks/bitvector/or_via_add.certir --out /tmp/q.certir
cargo run -p certiforge-cli -- package build benchmarks/bitvector/or_via_add.certir \
  --optimized /tmp/q.certir --out artifacts/demo
cargo run -p certiforge-cli -- package verify artifacts/demo   # ACCEPT | REJECT
cargo run -p certiforge-cli -- attack artifacts/demo
```

Windows: see `docs/WINDOWS_SETUP.md` and `scripts/bootstrap_windows.ps1`.

## Repository layout

| Path | Role |
|------|------|
| `crates/` | Rust workspace (CertIR, packages, ForgeOpt, CLI) |
| `formal/` | Lean 4 CertiForge semantics and theorems |
| `benchmarks/` | Bitvector / adversarial kernels |
| `docs/` | Research charter, TCB, literature, gaps |
| `artifacts/` | Example accepted packages |
| `results/` | Experiment outputs and Phase I verdict |

## Trust policy (summary)

| Component | Role |
|-----------|------|
| ForgeOpt / Z3 / CaDiCaL / AI | Untrusted proof *discovery* |
| Lean kernel + LRAT checker (`bv_check`/`bv_decide`) | Trusted checking (plus `Lean.ofReduceBool`) |
| Rust package verifier | Executable TCB (differential-tested vs Lean; not yet refined) |
| Tests / benchmarks | Empirical only — never universal correctness |

Full accounting: `docs/TCB.md`, `docs/AXIOM_AUDIT.md`.

## License

Apache-2.0 — see `LICENSE`.
