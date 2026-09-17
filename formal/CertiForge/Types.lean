/-
  Typing judgments for CertIR.
-/
import CertiForge.Syntax

namespace CertiForge

/-- Typing environment: variable → type. -/
abbrev Env := List (Var × Ty)

def Env.lookup (Γ : Env) (x : Var) : Option Ty :=
  match Γ with
  | [] => none
  | (y, t) :: rest => if x = y then some t else rest.lookup x

/-- Well-typedness of expressions (computable checker). -/
def typeOf (Γ : Env) : Expr → Option Ty
  | .constBool _ => some .bool
  | .constBv w _ => some (.bitvec w)
  | .var x => Γ.lookup x
  | .unop .not e =>
      match typeOf Γ e with
      | some .bool => some .bool
      | some (.bitvec w) => some (.bitvec w)
      | _ => none
  | .unop .neg e =>
      match typeOf Γ e with
      | some (.bitvec w) => some (.bitvec w)
      | _ => none
  | .binop op e1 e2 =>
      match typeOf Γ e1, typeOf Γ e2 with
      | some (.bitvec w1), some (.bitvec w2) =>
          if w1 = w2 then
            match op with
            | .add | .sub | .mul | .and | .or | .xor | .shl | .lshr =>
                some (.bitvec w1)
          else none
      | _, _ => none
  | .cmp _ e1 e2 =>
      match typeOf Γ e1, typeOf Γ e2 with
      | some (.bitvec w1), some (.bitvec w2) =>
          if w1 = w2 then some .bool else none
      | some .bool, some .bool => some .bool
      | _, _ => none
  | .select c t f =>
      match typeOf Γ c, typeOf Γ t, typeOf Γ f with
      | some .bool, some tyT, some tyF =>
          if tyT = tyF then some tyT else none
      | _, _, _ => none

/-- A program is well-typed when the body has the declared return type
    under the parameter environment. -/
def Program.wellTyped (P : Program) : Bool :=
  match typeOf P.params P.body with
  | some ty => decide (ty = P.retTy)
  | none => false

end CertiForge
