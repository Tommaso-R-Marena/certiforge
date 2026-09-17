# Research Charter — CERTIFORGE

## Central question

How much useful AI-generated software can be made independently machine-verifiable
without trusting the generating AI?

Stronger form: can optimization itself remain untrusted, such that arbitrary
transformations are admitted only with machine-checkable semantic equivalence proofs?

## Non-goals (Phase I)

- GUI / AI coding wrappers
- Claiming verification from unit tests alone
- Trusting SMT solvers as final authorities
- Arbitrary Rust/C++ support
- Closing the full source→machine-code gap

## Governing principle

The independently checkable artifact is the security boundary.

## Phase map

| Phase | Focus |
|-------|--------|
| I | Pure straight-line CertIR + packages + ForgeOpt v1 |
| II | Superoptimization study at corpus scale |
| III | Bounded control flow |
| IV | Effect manifests |
| V | Restricted source frontend |
| VI | Executable certification path |

## Evidence vocabulary

We distinguish: **formally proved**, **mechanically checked**, **bounded/model-checked**,
**empirically tested**, **assumed**, **trusted**, **unverified**. Never promote an
assumption to a result by silence.
