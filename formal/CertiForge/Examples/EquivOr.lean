import Std.Tactic.BVDecide

/-- Portable equivalence certificate for Phase I demo (u32 form). -/
theorem certiforge_and_xor_add_eq_or (x y : BitVec 32) :
    (x &&& y) + (x ^^^ y) = x ||| y := by
  bv_decide
