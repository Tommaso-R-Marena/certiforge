/-
  BitVec decision procedure experiments and portable certificate notes.

  Lean's `bv_decide` bit-blasts goals, invokes CaDiCaL (untrusted search),
  and checks an LRAT certificate inside Lean. The SAT solver is a proof
  *producer*, not a trusted authority.

  Axiom note: `bv_decide` introduces `Lean.ofReduceBool`, which trusts
  Lean's code generator. See docs/AXIOM_AUDIT.md.
-/
import Std.Tactic.BVDecide
import CertiForge.Syntax

namespace CertiForge.BitVecExamples

/-- Classic identity: (a &&& b) + (a ^^^ b) = a ||| b -/
theorem and_xor_eq_or (a b : BitVec 32) :
    (a &&& b) + (a ^^^ b) = a ||| b := by
  bv_decide

/-- Strength-reduction style: x + x = x <<< 1 -/
theorem add_self_eq_shl1 (x : BitVec 64) :
    x + x = x <<< 1 := by
  bv_decide

/-- Mask identity: x &&& 0 = 0 -/
theorem and_zero (x : BitVec 16) :
    x &&& 0#16 = 0#16 := by
  bv_decide

/-- XOR cancellation: x ^^^ x = 0 -/
theorem xor_self (x : BitVec 8) :
    x ^^^ x = 0#8 := by
  bv_decide

/-- Rotate-left-7 form equals itself (shift composition sanity). -/
theorem rotate_left_7_shift_form (x : BitVec 32) :
    (x <<< 7) ||| (x >>> 25) = (x <<< 7) ||| (x >>> 25) := by
  rfl

/-- Concrete counterexample: 1 + 1 ≠ 1 on BitVec 32.
    Demonstrates that intentionally unsound optimizer rules are rejected. -/
theorem one_plus_one_ne_one : ¬ (1#32 + 1#32 = 1#32) := by
  native_decide

end CertiForge.BitVecExamples
