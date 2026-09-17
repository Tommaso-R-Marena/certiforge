# Project State

## Strongest formally proved claim

If a package has `CheckedEvidence` (hashes ok, cert flags, `SemEq P Q`, `Satisfies P S`,
`Satisfies Q S`), then `PackageSemanticallyValid pkg` (`accepted_package_sound`).
Bitvector identities such as `(x&&y)+(x^^y)=x||y` are proved with `bv_decide`.

## Strongest executable result

`certiforge package verify` **ACCEPT**s a built package for `or_via_add` (u8) after ForgeOpt
rewrites to `or`, with hash integrity and substantive Lean cert placeholders + exhaustive
domain equivalence checking.

## Strongest optimization result

ForgeOpt admits `and_xor_add_to_or` with static cost 4→1 on u8; unsound candidates rejected.
Also admits `xor_self` and `and_zero` strength reductions under the weighted cost model.

## Strongest adversarial result

After fixing certificate-quality checking: **0 false accepts / 7 attacks** on the suite
(`modify_optimized/program/spec`, truncate cert, hash sub, drop equiv flag, corrupt proof).

## Current TCB

Lean kernel + `ofReduceBool` + Rust verifier + SHA-256 + parser. See `docs/TCB.md`.

## Known unsoundness / gaps

- Rust↔Lean semantics not proved (differential tests only).
- Sampled equivalence on large domains is incomplete without a real Lean/LRAT equiv theorem
  tied to the concrete program AST (Phase I requires substantive cert text + samples).
- Spec-intent gap remains (`docs/SPECIFICATION_GAP.md`).
- No executable/native chain (`docs/EXECUTABLE_GAP.md`).
- Effects are stubs.

## Failed claims

None claimed then withdrawn yet. See `docs/FAILED_CLAIMS.md` for the certificate-quality
bug found by adversarial testing (fixed; recorded).

## Benchmark status

Smoke suite: multiple admitted opts; see `certiforge benchmark`.

## Next three highest-information experiments

1. Wire real `bv_check` LRAT files into packages for u32 kernels (not placeholders).
2. Differential fuzz Lean `#eval` vs Rust interpreter on random CertIR.
3. Expand mutation campaign; track false-acceptance over ≥10³ mutants.
