import Std.Tactic.BVDecide
import CertiForge.Semantics
import CertiForge.Equivalence
import CertiForge.Soundness

namespace CertiForge.Examples.SwapAdd

open CertiForge

/-- P: (x & y) + (x ^ y) -/
def programP : Program where
  params := [("x", .bitvec .w32), ("y", .bitvec .w32)]
  body :=
    .binop .add
      (.binop .and (.var "x") (.var "y"))
      (.binop .xor (.var "x") (.var "y"))
  retTy := .bitvec .w32

/-- Q: x | y  — equivalent and cheaper. -/
def programQ : Program where
  params := [("x", .bitvec .w32), ("y", .bitvec .w32)]
  body := .binop .or (.var "x") (.var "y")
  retTy := .bitvec .w32

theorem programs_wellTyped : programP.wellTyped = true ∧ programQ.wellTyped = true := by
  native_decide

/-- Bit-level equivalence of the bodies on BitVec 32. -/
theorem body_equiv (x y : BitVec 32) :
    (x &&& y) + (x ^^^ y) = x ||| y := by
  bv_decide

end CertiForge.Examples.SwapAdd
