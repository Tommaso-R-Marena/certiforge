/-
  Operational semantics for CertIR.
  Fixed-width bitvector operations use Lean's `BitVec` with wrapping arithmetic.
-/
import CertiForge.Syntax
import CertiForge.Types

namespace CertiForge

/-- Runtime values. -/
inductive Value where
  | bool  : Bool → Value
  | bitvec : (w : Width) → BitVec w.toNat → Value
  deriving Repr

/-- Store mapping variables to values. -/
abbrev Store := List (Var × Value)

def Store.lookup (σ : Store) (x : Var) : Option Value :=
  match σ with
  | [] => none
  | (y, v) :: rest => if x = y then some v else rest.lookup x

/-- Mask a natural into a width. -/
def maskNat (w : Width) (n : Nat) : Nat :=
  n % (2 ^ w.toNat)

/-- Evaluate a binary bitvector operation. -/
def evalBinOp {n : Nat} (op : BinOp) (a b : BitVec n) : BitVec n :=
  match op with
  | .add  => a + b
  | .sub  => a - b
  | .mul  => a * b
  | .and  => a &&& b
  | .or   => a ||| b
  | .xor  => a ^^^ b
  | .shl  => a <<< b.toNat
  | .lshr => a >>> b.toNat

/-- Evaluate a comparison on bitvectors (unsigned). -/
def evalCmpBv {n : Nat} (op : CmpOp) (a b : BitVec n) : Bool :=
  match op with
  | .eq  => a == b
  | .ne  => !(a == b)
  | .ult => a < b
  | .ule => a ≤ b
  | .ugt => b < a
  | .uge => b ≤ a

/-- Evaluate a comparison on booleans. -/
def evalCmpBool (op : CmpOp) (a b : Bool) : Option Bool :=
  match op with
  | .eq => some (a == b)
  | .ne => some (a != b)
  | _   => none

/-- Expression evaluation. Returns `none` on type/runtime errors. -/
partial def evalExpr (σ : Store) : Expr → Option Value
  | .constBool b => some (.bool b)
  | .constBv w n =>
      some (.bitvec w (BitVec.ofNat w.toNat (maskNat w n)))
  | .var x => σ.lookup x
  | .unop .not e =>
      match evalExpr σ e with
      | some (.bool b) => some (.bool (!b))
      | some (.bitvec w v) => some (.bitvec w (~~~v))
      | _ => none
  | .unop .neg e =>
      match evalExpr σ e with
      | some (.bitvec w v) => some (.bitvec w (-v))
      | _ => none
  | .binop op e1 e2 =>
      match evalExpr σ e1, evalExpr σ e2 with
      | some (.bitvec w1 a), some (.bitvec w2 b) =>
          if h : w1 = w2 then
            let b' : BitVec w1.toNat := by
              rw [h]; exact b
            some (.bitvec w1 (evalBinOp op a b'))
          else none
      | _, _ => none
  | .cmp op e1 e2 =>
      match evalExpr σ e1, evalExpr σ e2 with
      | some (.bitvec w1 a), some (.bitvec w2 b) =>
          if h : w1 = w2 then
            let b' : BitVec w1.toNat := by
              rw [h]; exact b
            some (.bool (evalCmpBv op a b'))
          else none
      | some (.bool a), some (.bool b) =>
          (evalCmpBool op a b).map Value.bool
      | _, _ => none
  | .select c t f =>
      match evalExpr σ c with
      | some (.bool true)  => evalExpr σ t
      | some (.bool false) => evalExpr σ f
      | _ => none

/-- Build an initial store from a parameter list and input values.
    Lengths and types must match; otherwise `none`. -/
def bindInputs (params : List (Var × Ty)) (inputs : List Value) : Option Store :=
  match params, inputs with
  | [], [] => some []
  | (x, ty) :: ps, v :: vs =>
      let ok :=
        match ty, v with
        | .bool, .bool _ => true
        | .bitvec w, .bitvec w' _ => decide (w = w')
        | _, _ => false
      if ok then
        match bindInputs ps vs with
        | some σ => some ((x, v) :: σ)
        | none => none
      else none
  | _, _ => none

/-- Program evaluation: `eval P inputs = some output` or `none` on error. -/
def eval (P : Program) (inputs : List Value) : Option Value :=
  match bindInputs P.params inputs with
  | some σ =>
      match evalExpr σ P.body with
      | some v =>
          -- Check return type matches
          let ok :=
            match P.retTy, v with
            | .bool, .bool _ => true
            | .bitvec w, .bitvec w' _ => decide (w = w')
            | _, _ => false
          if ok then some v else none
      | none => none
  | none => none

end CertiForge
