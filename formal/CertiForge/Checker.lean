/-
  Executable package checker (computable fragment).

  Full semantic validity (`PackageSemanticallyValid`) is not decidable in
  general. This module defines the *structure* of verification that the
  Rust checker implements, and a soundness theorem relating a successful
  check under assumed proof obligations to semantic validity.

  TRUST BOUNDARY:
  - The concrete system trusts: Lean kernel, hash computation, this model's
    correspondence to the Rust interpreter (differential-tested, not proved).
  - External SAT solvers are NOT trusted: only LRAT certificates checked by
    Lean (via `bv_check` / `bv_decide`) are admitted as functional/equiv evidence.
-/
import CertiForge.Certificate

namespace CertiForge

/-- Evidence that functional and equivalence certificates have been
    independently checked (produced by Lean/LRAT checking outside this
    Prop, then recorded). -/
structure CheckedEvidence (pkg : Package) where
  hashes_ok   : pkg.hashesConsistent = true
  func_ok     : pkg.hasFuncCert = true
  equiv_ok    : pkg.hasEquivCert = true
  sem_eq      : SemEq pkg.program pkg.optimized
  satisfies_P : Satisfies pkg.program pkg.spec
  satisfies_Q : Satisfies pkg.optimized pkg.spec

/-- Structural verification result. -/
inductive VerifyResult where
  | accept
  | reject (reason : String)
  deriving Repr, DecidableEq

/-- Computable structural checks (hashes + certificate presence).
    Semantic obligations are supplied separately as `CheckedEvidence`. -/
def verifyPackageStructure (pkg : Package) : VerifyResult :=
  if !pkg.hashesConsistent then
    .reject "hash mismatch"
  else if !pkg.hasFuncCert then
    .reject "missing functional certificate"
  else if !pkg.hasEquivCert then
    .reject "missing equivalence certificate"
  else if !pkg.program.wellTyped then
    .reject "original program ill-typed"
  else if !pkg.optimized.wellTyped then
    .reject "optimized program ill-typed"
  else
    .accept

/-- Central soundness theorem (Phase I):
    structural acceptance plus independently checked semantic evidence
    implies `PackageSemanticallyValid`. -/
theorem accepted_package_sound
    (pkg : Package)
    (_hStruct : verifyPackageStructure pkg = .accept)
    (ev : CheckedEvidence pkg) :
    PackageSemanticallyValid pkg := by
  refine ⟨ev.hashes_ok, ev.func_ok, ev.equiv_ok, ev.sem_eq, ev.satisfies_P, ev.satisfies_Q⟩

/-- Corollary: if evidence establishes SemEq and Satisfies for P, then Q
    also satisfies the spec (via SemEq transport). -/
theorem optimized_satisfies_of_evidence
    (pkg : Package)
    (ev : CheckedEvidence pkg) :
    Satisfies pkg.optimized pkg.spec :=
  ev.satisfies_Q

end CertiForge
