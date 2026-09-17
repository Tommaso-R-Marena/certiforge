# Literature Audit

Primary sources preferred. This audit informs novelty claims; it is not exhaustive of all FM literature.

## Proof-Carrying Code (PCC)

| Field | Notes |
|-------|-------|
| Problem | Untrusted code carries a proof checked by a small consumer |
| Mechanism | Formal proofs of safety/type properties checked by a trusted checker |
| TCB | Checker + logic; generator untrusted |
| Proof-producing vs solver-trusting | Classical PCC is proof-carrying |
| Optimization story | Not centered on untrusted superopt |
| AI story | Predates LLM code generation |
| What CERTIFORGE adds | PCC-style packages for AI code + untrusted superopt with equivalence certs + explicit false-acceptance metric |

Key refs: Necula & Lee (PCC); Appel (Foundational PCC).

## Proof-Carrying Authorization (PCA)

| Field | Notes |
|-------|-------|
| Problem | Distributed authorization with checkable proofs |
| Relevance | Motivates effect/authority manifests (Phase IV), not Phase I core |
| What CERTIFORGE adds | Bridge from abstract effect traces to future authority kernels |

## CompCert

| Field | Notes |
|-------|-------|
| Problem | Formally verified C compiler (Coq) |
| Language | Clight → assembly |
| Mechanism | Correctness theorems for compilation passes |
| TCB | Coq kernel + unverified parts of toolchain carefully scoped |
| Optimization | Verified optimizations inside the compiler |
| What CERTIFORGE adds | Orthogonal: we keep *search* untrusted and certify candidates; CompCert trusts/proves the optimizer itself |

## CakeML

| Field | Notes |
|-------|-------|
| Problem | Verified ML dialect with verified compiler |
| Mechanism | HOL4 end-to-end |
| What CERTIFORGE adds | Different entry point (AI packages + CertIR), not a verified general compiler |

## Lean-based verified compilation

Ongoing work (e.g. Lean4lean, various backends). CERTIFORGE uses Lean as semantic reference + `bv_decide` LRAT checking, not as a full verified compiler (yet).

## F* / Dafny / Verus / Creusot

| System | Role | Trust | Gap vs CERTIFORGE |
|--------|------|-------|-------------------|
| F* | Dependent types + SMT | Often SMT-trusting for discharge | Not AI-package + untrusted superopt architecture |
| Dafny | Spec+verify → C#/etc | Boogie/Z3 | Same |
| Verus | Rust verification | SMT | Complementary Phase V+; document as layer |
| Creusot | Rust → Why3 | Why3/SMT | Complementary |

## Kani / CBMC

Bounded model checking for Rust/C. Excellent counterexample engines. **Not** unbounded certificates. CERTIFORGE treats them as complementary evidence layers (`docs/VERIFICATION_LAYERS.md`).

## Alive / Alive2

| Field | Notes |
|-------|-------|
| Problem | Translation validation / peephole correctness for LLVM |
| Mechanism | SMT over LLVM semantics; counterexamples |
| Trust | Solver-trusting unless proof objects exported |
| What CERTIFORGE adds | CertIR-level Lean certificates; Alive2 as optional cross-check (Phase VI), not TCB by default |

## Souper / STOKE / superoptimization

| System | Notes |
|--------|-------|
| Souper | LLVM superopt via synthesis/SMT; optimizer trust or TV |
| STOKE | Stochastic superopt for x86; testing/MCMC; not PCC packages |
| egg / egglog | Equality saturation; powerful discovery, not a small checker |

CERTIFORGE's ForgeOpt may use these *ideas* as untrusted search; admission requires independent certificates.

## Translation validation

Validates each compilation run rather than proving the compiler. Closely related to our P≡Q certificates. CERTIFORGE specializes this to AI packages + adversarial false-acceptance evaluation + effect manifests roadmap.

## WebAssembly formal semantics

Wasm has formal specs and verified interpreters (e.g. WasmCert). Candidate backend for Phase VI (`docs/BACKEND_DECISION.md` deferred until Phase I data).

## Proof-generating / proof-carrying synthesis

Program synthesis with certificates exists in various forms (Fiat, verified synthesizers). LLM-era packaging with untrusted superopt + false-acceptance campaigns is less standardized.

## Neural / LLM code verification & repair

Rapidly growing (LLM + RAG + tools + verifiers). Most systems use tests/analyzers/SMT as oracles without small independent PCC packages or proof-preserving superoptimization.

## AI-generated GPU/kernel verification

Specialized (e.g. kernel correctness with SMT/ITPs). Out of Phase I scope; relevant later for high-value kernels.

## Summary table (selected)

| System | Func props | Effects | Opt story | AI story | Checker small? |
|--------|------------|---------|-----------|----------|----------------|
| PCC | Safety-oriented | Limited | Limited | No | Yes (goal) |
| CompCert | Compilation | N/A | Verified opts | No | Kernel |
| Alive2 | LLVM peephole | N/A | TV | No | Solver-heavy |
| STOKE | Testing | N/A | Stochastic | No | No |
| Verus/Kani | Rust | Partial | N/A | Optional | SMT/BMC |
| **CERTIFORGE** | CertIR specs | Phase IV | Untrusted+cert | Central | Goal |

## Already-occupied claims (do not overclaim)

- PCC itself is classical.
- Translation validation / Alive-style peephole checking exists.
- Verified compilers exist.
- Bitvector decision procedures with LRAT in Lean exist (`bv_decide`).

CERTIFORGE must claim a **combination** and evaluation methodology, not reinvent PCC or `bv_decide`.
