/-
  Certificate and package structures.
-/
import CertiForge.Syntax
import CertiForge.Spec
import CertiForge.Equivalence

namespace CertiForge

/-- Content hashes are opaque natural numbers in the formal model
    (concrete packages use SHA-256 hex strings). -/
abbrev Hash := Nat

/-- A Phase-I certificate package (pure, no effects). -/
structure Package where
  program          : Program
  optimized        : Program
  spec             : Spec
  programHash      : Hash
  optimizedHash    : Hash
  specHash         : Hash
  /-- Claimed functional proof flag (concrete system checks Lean/LRAT artifacts). -/
  hasFuncCert      : Bool
  /-- Claimed equivalence proof flag. -/
  hasEquivCert     : Bool
  /-- Whether hashes are consistent with contents (modeled as a Bool here;
      the executable checker computes SHA-256). -/
  hashesConsistent : Bool
  -- No `deriving Repr`: `Spec` contains `Prop` fields.

/-- Semantic validity of an accepted package. -/
def PackageSemanticallyValid (pkg : Package) : Prop :=
  pkg.hashesConsistent = true ∧
  pkg.hasFuncCert = true ∧
  pkg.hasEquivCert = true ∧
  SemEq pkg.program pkg.optimized ∧
  Satisfies pkg.program pkg.spec ∧
  Satisfies pkg.optimized pkg.spec

end CertiForge
