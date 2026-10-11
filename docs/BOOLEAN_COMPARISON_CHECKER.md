# Reject Boolean ordering before optimization

The interpreter supports Boolean equality and inequality, but not unsigned Boolean
ordering. Previously `Program::check` accepted all six comparison operators on
Booleans. An accepted expression such as
`and(select(ult(false, true), u8(7), u8(7)), u8(0))` therefore failed with
`bad bool cmp`, while replacing it by `u8(0)` succeeded. This violates preservation
of observable error behavior.

The checker now accepts only `eq` and `ne` for Boolean operands. All six unsigned
comparison operators remain available for matching bit-vector widths. Package
construction and verification already check both programs, so the erroneous input
is rejected before optimization can receive verification evidence.

Regression tests cover every Boolean comparison, all four bit-vector widths, and
the actual interpreter's error-versus-value counterexample. The interpreter and
the width implementations are unchanged. The new PCS Lean 4.28 refinement package
also contains a kernel-checked model of this counterexample and a repaired checker;
its source extraction covers `eval_expr` and its callees, not the entire Rust
frontend, optimizer, package verifier, or top-level `eval` loop. Certificate inputs
must remain well typed and canonical. No sampled equivalence claim is upgraded to
a universal proof by this repair.
