/-
  Soundness re-exports and Phase-I metatheorems.
-/
import CertiForge.Checker
import CertiForge.Equivalence

namespace CertiForge

/-- If P ≡ Q and P satisfies S, the optimized program satisfies S.
    This is the key lemma justifying proof-preserving superoptimization. -/
theorem proof_preserving_optimization
    (P Q : Program) (S : Spec)
    (heq : SemEq P Q)
    (hsat : Satisfies P S) :
    Satisfies Q S :=
  satisfies_of_semEq heq hsat

/-- Rejecting packages with inconsistent hashes is mandatory for acceptance. -/
theorem reject_inconsistent_hashes
    (pkg : Package)
    (h : pkg.hashesConsistent = false) :
    verifyPackageStructure pkg ≠ .accept := by
  intro hAccept
  simp [verifyPackageStructure, h] at hAccept

/-- Rejecting packages without functional certificates. -/
theorem reject_missing_func
    (pkg : Package)
    (hHash : pkg.hashesConsistent = true)
    (h : pkg.hasFuncCert = false) :
    verifyPackageStructure pkg ≠ .accept := by
  intro hAccept
  simp [verifyPackageStructure, hHash, h] at hAccept

/-- Rejecting packages without equivalence certificates. -/
theorem reject_missing_equiv
    (pkg : Package)
    (hHash : pkg.hashesConsistent = true)
    (hFunc : pkg.hasFuncCert = true)
    (h : pkg.hasEquivCert = false) :
    verifyPackageStructure pkg ≠ .accept := by
  intro hAccept
  simp [verifyPackageStructure, hHash, hFunc, h] at hAccept

end CertiForge
