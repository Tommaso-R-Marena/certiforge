# Trusted Computing Base (TCB)

Last updated: Phase I implementation snapshot.

## Trusted (executable acceptance path)

| Component | Why trusted | Size / notes |
|-----------|-------------|--------------|
| Lean 4 kernel | Proof checking | External toolchain |
| `bv_decide` LRAT checker (in Lean) | Checks SAT proofs | Verified checker algorithms in Lean; see axioms |
| Lean code generator via `Lean.ofReduceBool` | Used by `bv_decide` / `native_decide` | **Documented trust** — not "axiom-free" |
| Rust `certiforge package verify` | Hash + typing + equiv/functional checks + cert quality | Part of Phase I executable TCB |
| SHA-256 implementation (`sha2` crate) | Hash integrity | Standard crypto lib |
| CertIR parser (rejection of bad syntax) | Must not misparse into different programs | Differential/roundtrip tested |

## Untrusted (discovery / search)

| Component | Role |
|-----------|------|
| ForgeOpt rewrite + stochastic search | Propose candidates |
| Intentionally unsound rewrite rules | Red-team the checker |
| CaDiCaL (via `bv_decide`) | Produce LRAT proofs |
| Z3 (installed; optional experiments) | Not on Phase I accept path |
| AI / Cursor | May propose programs; never verification |
| Benchmarks / fuzzing | Empirical only |

## Formally proved (Lean)

- `SemEq` refl/symm/trans
- `satisfies_of_semEq` / `proof_preserving_optimization`
- `accepted_package_sound` (structural accept + `CheckedEvidence` ⇒ `PackageSemanticallyValid`)
- Bitvector identities via `bv_decide` (e.g. `(x∧y)+(x⊕y)=x∨y`)
- Reject lemmas for missing hashes/certs

## Mechanically checked but not refined

- Correspondence between Lean `eval` and Rust interpreter: **differential tests only**
- Package verifier's exhaustive/sampled equivalence vs Lean `SemEq`: **not proved**

## Assumed / out of scope (Phase I)

- OS correctness, CPU correctness
- That a human-written/AI-written **spec matches user intent** (`docs/SPECIFICATION_GAP.md`)
- Native executable semantics (`docs/EXECUTABLE_GAP.md`)
- Effect traces (stub only)

## Metric

See `scripts/tcb_report.sh` and `results/tcb_metrics.json`.
Do not game by moving code across the boundary without documentation.
