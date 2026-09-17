/-
  Semantic equivalence of CertIR programs.
-/
import CertiForge.Semantics
import CertiForge.Spec

namespace CertiForge

/-- Two programs are semantically equivalent when they agree on all inputs
    (including both returning `none` for the same ill-formed inputs). -/
def SemEq (P Q : Program) : Prop :=
  ∀ inputs, eval P inputs = eval Q inputs

theorem SemEq.refl (P : Program) : SemEq P P := by
  intro inputs; rfl

theorem SemEq.symm {P Q : Program} (h : SemEq P Q) : SemEq Q P := by
  intro inputs; simp [h inputs]

theorem SemEq.trans {P Q R : Program}
    (h1 : SemEq P Q) (h2 : SemEq Q R) : SemEq P R := by
  intro inputs
  rw [h1, h2]

/-- If `P ≡ Q` and `P` satisfies `S`, then `Q` satisfies `S`. -/
theorem satisfies_of_semEq {P Q : Program} {S : Spec}
    (heq : SemEq P Q) (hsat : Satisfies P S) : Satisfies Q S := by
  intro inputs hpre
  obtain ⟨output, heval, hpost⟩ := hsat inputs hpre
  refine ⟨output, ?_, hpost⟩
  rw [← heq]
  exact heval

end CertiForge
