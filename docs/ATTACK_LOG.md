# Attack Log

## 2026-09-17 — Phase I adversarial suite

Baseline package: `artifacts/or_via_add` (ForgeOpt `and_xor_add_to_or`).

| Attack | Result | Notes |
|--------|--------|-------|
| modify_optimized | rejected | Hash + equivalence |
| modify_program | rejected | Hash mismatch |
| modify_spec_vacuous | rejected | Vacuity audit |
| truncate_certificate | rejected | Hash + cert quality |
| hash_substitution | rejected | Hash mismatch |
| drop_equiv_flag | rejected | Manifest flag |
| corrupt_proof_text | rejected (after fix) | See bug below |

### Bug: corrupt proof accepted when hashes updated (FIXED)

**Symptom:** Replacing `certificates/equivalence.lean` with `axiom unsound : False` and
updating the manifest hash produced **FALSE_ACCEPT** on u8 packages because exhaustive
interpreter equivalence was treated as sufficient without substantive cert quality checks.

**Root cause:** Cert quality gate only applied to non-exhaustive (u32/u64) domains.

**Fix:** Always require substantive Lean certificate text when cert flags are set; reject
`axiom unsound` patterns.

**Regression:** Covered by `certiforge attack` and package tests.

False acceptance count after fix: **0 / 7**.
