/-
  Specifications and functional correctness for CertIR programs.
-/
import CertiForge.Semantics

namespace CertiForge

/-- A functional specification over inputs and optional outputs. -/
structure Spec where
  /-- Precondition on inputs. -/
  pre  : List Value → Prop
  /-- Postcondition relating inputs to output. -/
  post : List Value → Value → Prop

/-- Program `P` satisfies specification `S`. -/
def Satisfies (P : Program) (S : Spec) : Prop :=
  ∀ inputs,
    S.pre inputs →
    ∃ output, eval P inputs = some output ∧ S.post inputs output

/-- Exact functional equivalence as a specification relative to a reference
    program `ref`. -/
def Spec.equivTo (ref : Program) : Spec where
  pre  := fun inputs => (eval ref inputs).isSome
  post := fun inputs output => eval ref inputs = some output

/-- Vacuous (impossible) precondition — any program "satisfies" this,
    which is why vacuity audits are required. -/
def Spec.vacuous : Spec where
  pre  := fun _ => False
  post := fun _ _ => True

/-- A specification is suspicious if its precondition is unsatisfiable
    (detected for common patterns; full undecidable check is not claimed). -/
def Spec.isObviouslyVacuous (S : Spec) : Prop :=
  ∀ inputs, ¬ S.pre inputs

end CertiForge
