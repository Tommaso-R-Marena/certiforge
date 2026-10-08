import CertiForge.Semantics
import Std.Tactic.BVDecide

namespace CertiForge.Examples.BoundDemo

/-- These definitions bind the proof to complete CertIR ASTs, not a `True` stub. -/
def original : Program := {
  params := [("x", .bitvec .w8), ("y", .bitvec .w8)]
  body := .binop .add (.binop .and (.var "x") (.var "y"))
                       (.binop .xor (.var "x") (.var "y"))
  retTy := .bitvec .w8 }

def optimized : Program := {
  params := [("x", .bitvec .w8), ("y", .bitvec .w8)]
  body := .binop .or (.var "x") (.var "y")
  retTy := .bitvec .w8 }

/-- All well-typed u8 input pairs, in the Lean semantics. No Rust refinement claim. -/
theorem typed_input_equivalence (x y : BitVec 8) :
    eval original [.bitvec .w8 x, .bitvec .w8 y] =
    eval optimized [.bitvec .w8 x, .bitvec .w8 y] := by
  have identity : (x &&& y) + (x ^^^ y) = x ||| y := by bv_decide
  simp [original, optimized, eval, bindInputs, evalExpr, Store.lookup, evalBinOp, identity]

/-- The optimized AST computes the independently stated OR specification. -/
theorem typed_output_specification (x y : BitVec 8) :
    eval optimized [.bitvec .w8 x, .bitvec .w8 y] = some (.bitvec .w8 (x ||| y)) := by
  simp [optimized, eval, bindInputs, evalExpr, Store.lookup, evalBinOp]

-- The actual dependency set is emitted by the build and retained in evidence.
#print axioms typed_input_equivalence
#print axioms typed_output_specification

end CertiForge.Examples.BoundDemo
