# Verification Layers

| Property | Tool | Completeness | Trust | Bounded? | Artifact |
|----------|------|--------------|-------|----------|----------|
| BitVec identity | Lean `bv_decide` | Complete for supported fragment | LRAT checker + `ofReduceBool` | N/A (decidable BV) | `.lean` / `.lrat` |
| Package structure | Rust verifier | Structural | Rust TCB | N/A | ACCEPT/REJECT |
| Equivalence (Bool/u8/u16, Cartesian size ≤65,536) | Exhaustive Rust replay | Complete on admitted domain | Rust parser/interpreter | Finite domain | JSON receipt, exact artifact hashes |
| Package equivalence (u32/u64 or larger Cartesian domains) | Rejected | Not supported | No acceptance | N/A | UNVERIFIED_LARGE_DOMAIN |
| Vacuity audit | Heuristic | Incomplete | Diagnostic | N/A | reject reasons |
| Rust panics/overflow (future) | Kani | Bounded | BMC | Yes | Kani report |
| Rust proofs (future) | Verus | Spec-dependent | SMT | Usually | Verus proof |

Do not equate Kani/Verus guarantees with Lean proofs.

Package acceptance never asserts a Lean kernel check. The separate BoundDemo proofs bind complete u8 ASTs in Lean; neither a general package-certificate verifier nor Rust/Lean refinement follows from them.
