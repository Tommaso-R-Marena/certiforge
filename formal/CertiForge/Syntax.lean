/-
  CertIR v0 syntax — pure straight-line bitvector IR.
  Canonical semantic reference lives in this Lean development.
-/
namespace CertiForge

/-- Fixed-width unsigned bitvector kinds supported in Phase I. -/
inductive Width where
  | w8
  | w16
  | w32
  | w64
  deriving DecidableEq, Repr, Inhabited

/-- Map a width tag to its bit width. -/
def Width.toNat : Width → Nat
  | .w8  => 8
  | .w16 => 16
  | .w32 => 32
  | .w64 => 64

/-- CertIR types: booleans and fixed-width bitvectors. -/
inductive Ty where
  | bool
  | bitvec (w : Width)
  deriving DecidableEq, Repr, Inhabited

/-- Binary operators on bitvectors (wrapping semantics). -/
inductive BinOp where
  | add | sub | mul
  | and | or  | xor
  | shl | lshr
  deriving DecidableEq, Repr, Inhabited

/-- Comparison operators producing `Bool`. -/
inductive CmpOp where
  | eq | ne | ult | ule | ugt | uge
  deriving DecidableEq, Repr, Inhabited

/-- Unary operators. -/
inductive UnOp where
  | not          -- bitwise / boolean not
  | neg          -- two's-complement negation (bitvec only)
  deriving DecidableEq, Repr, Inhabited

/-- Variable names are strings in the formal model. -/
abbrev Var := String

/-- Expressions of CertIR (pure, straight-line). -/
inductive Expr where
  | constBool  (b : Bool)
  | constBv    (w : Width) (n : Nat)
  | var        (x : Var)
  | unop       (op : UnOp) (e : Expr)
  | binop      (op : BinOp) (e1 e2 : Expr)
  | cmp        (op : CmpOp) (e1 e2 : Expr)
  | select     (cond t f : Expr)
  deriving Repr, Inhabited

/-- A CertIR program: parameters with types, body expression, result type. -/
structure Program where
  params : List (Var × Ty)
  body   : Expr
  retTy  : Ty
  deriving Repr, Inhabited

end CertiForge
