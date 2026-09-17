# Research Log

## 2026-09-17 — Phase I bootstrap

**Hypothesis:** A small CertIR + Lean semantics + Rust package verifier can demonstrate
proof-preserving superoptimization with independent checking.

**Experiment:** Implement CertIR, Lean model, ForgeOpt, package verify, attack suite.

**Result:** ACCEPT on optimized `or_via_add`; unsound opts rejected; initially 1/7 false
accept on corrupt cert (fixed); then 0/7. `bv_decide` axioms include `Lean.ofReduceBool`.

**Interpretation:** Phase I core is viable; executable/Lean refinement and real LRAT
packaging remain gaps.

**Next:** Differential Lean↔Rust fuzz; u32 LRAT artifacts; expand mutants.
