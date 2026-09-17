# Supported Language (Phase I)

## CertIR surface syntax

```text
fn name(x: u32, y: u32) -> u32 {
  add(and(x, y), xor(x, y))
}
```

### Types

- `bool`
- `u8` | `u16` | `u32` | `u64`

### Expressions

- Constants: `true`, `false`, `u32(42)`
- Variables
- Unary: `not(e)`, `neg(e)`
- Binary: `add`, `sub`, `mul`, `and`, `or`, `xor`, `shl`, `lshr`
- Compare: `eq`, `ne`, `ult`, `ule`, `ugt`, `uge`
- `select(cond, t, f)`

### Semantics

- Pure, straight-line, wrapping bitvector arithmetic
- Shifts: if shift amount ≥ width, result is 0 (aligned with Lean `BitVec` shift-by-Nat behavior used here)
- No heap, I/O, concurrency, recursion, or unbounded loops

### Unsupported (must reject)

Any construct outside the grammar — no silent approximation.

### Source frontend

Restricted Rust-like frontend is **not** Phase I. See roadmap Phase V.
