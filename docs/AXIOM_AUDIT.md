# Axiom Audit

## Command

```bash
cd formal
lake env lean CertiForge/AxiomAudit.lean
```

## Results (Lean 4.16.0)

### `CertiForge.BitVecExamples.and_xor_eq_or`

Depends on axioms: `propext`, `Classical.choice`, `Lean.ofReduceBool`, `Quot.sound`.

### `CertiForge.BitVecExamples.add_self_eq_shl1`

Same: `propext`, `Classical.choice`, `Lean.ofReduceBool`, `Quot.sound`.

### `CertiForge.Examples.SwapAdd.body_equiv`

Same axiom set (via `bv_decide`).

### `CertiForge.accepted_package_sound`

At the integrated revision the actual audit reports `propext` and `Quot.sound`. The theorem is a structural implication from supplied `CheckedEvidence`; it does not establish that the Rust verifier constructs that evidence.

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

## AST-bound demo and bounded shifts

`Examples.BoundDemo.typed_input_equivalence` depends on `propext`, `Classical.choice`, `Lean.ofReduceBool`, and `Quot.sound`. `typed_output_specification` uses `propext` and `Quot.sound`. The shift-preservation dependency sets are printed by the same audit command. In particular, the computational SAT/LRAT path retains the documented code-generator trust; it is not silently imported into PCS scientific authority.
