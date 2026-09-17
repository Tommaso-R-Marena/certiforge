# Axiom Audit

## Command

```bash
cd formal
lake env lean scripts/axiom_audit.lean   # or the checked-in audit file
```

## Results (Lean 4.16.0)

### `CertiForge.BitVecExamples.and_xor_eq_or`

Depends on axioms: `propext`, `Classical.choice`, `Lean.ofReduceBool`, `Quot.sound`.

### `CertiForge.BitVecExamples.add_self_eq_shl1`

Same: `propext`, `Classical.choice`, `Lean.ofReduceBool`, `Quot.sound`.

### `CertiForge.Examples.SwapAdd.body_equiv`

Same axiom set (via `bv_decide`).

### `CertiForge.accepted_package_sound`

**Does not depend on any axioms** (pure structural implication from `CheckedEvidence`).

## Interpretation

- `bv_decide` uses an **untrusted** SAT solver (CaDiCaL) to *search* for an LRAT proof;
  Lean checks the certificate. The solver is **not** a trusted authority.
- `Lean.ofReduceBool` means **Lean’s code generator is in the TCB** for these proofs.
  We do **not** claim “no axioms” or “fully verified without compiler trust.”
- Standard Lean axioms (`propext`, `Quot.sound`, `Classical.choice`) appear as usual.

## Portable artifacts

Phase I stores Lean theorem files in packages. Full `bv_check file.lrat` portable LRAT
workflows are supported by Lean; packaging real LRAT blobs for every kernel is ongoing.
Stubs are labeled as stubs and must not be treated as checked UNSAT proofs.
