# Verification Layers

| Property | Tool | Completeness | Trust | Bounded? | Artifact |
|----------|------|--------------|-------|----------|----------|
| BitVec identity | Lean `bv_decide` | Complete for supported fragment | LRAT checker + `ofReduceBool` | N/A (decidable BV) | `.lean` / `.lrat` |
| Package structure | Rust verifier | Structural | Rust TCB | N/A | ACCEPT/REJECT |
| Equivalence (u8/u16) | Exhaustive interp | Complete on domain | Rust interp | Finite domain | samples |
| Equivalence (u32/u64) | Samples + Lean cert | Incomplete unless Lean covers | Mixed | Sampled | cert files |
| Vacuity audit | Heuristic | Incomplete | Diagnostic | N/A | reject reasons |
| Rust panics/overflow (future) | Kani | Bounded | BMC | Yes | Kani report |
| Rust proofs (future) | Verus | Spec-dependent | SMT | Usually | Verus proof |

Do not equate Kani/Verus guarantees with Lean proofs.
