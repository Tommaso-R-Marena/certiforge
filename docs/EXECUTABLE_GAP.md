# Executable Gap

Phase I proves/checks properties of **CertIR** semantics (Lean) and runs programs on the
**Rust interpreter**.

The following are **not** established:

| Arrow | Status |
|-------|--------|
| Restricted source → CertIR | Not yet (Phase V) |
| CertIR → LLVM/Wasm/asm | Not yet (Phase VI) |
| CertIR Lean eval ≡ Rust eval | Tested, not proved |
| Native execution ≡ CertIR | Unverified |

Until a backend strategy is evidenced, do not claim executable-level guarantees.
Decision record: `docs/BACKEND_DECISION.md` (deferred).
