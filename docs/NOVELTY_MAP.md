# Novelty Map

## Candidate contribution (tested against prior work)

An end-to-end architecture for AI-produced software packages containing independently
machine-checkable functional (and later effect) guarantees, combined with an untrusted
automatic superoptimizer whose transformations are admitted only after proof-producing
or independently checked semantic equivalence.

## Dimension audit

| ID | Claim | Prior art? | Residual novelty |
|----|-------|------------|------------------|
| A | Common certificate architecture (spec + func + effect + equiv) | Partial (PCC, TV, package formats) | Unified AI-oriented package + provenance invalidation + vacuity audits |
| B | Proof-preserving AI-directed superoptimization | Superopt + TV exist separately | Explicit untrusted AI/search + admission only via certificates; intentional unsound rules demo |
| C | Small checker vs large generator | Classical PCC goal | Measure `size(untrusted)/size(trusted)` on this stack; Lean LRAT path |
| D | Adversarial **false acceptance** benchmark for AI software packages | Mutation testing; less common as primary PCC metric | First-class false-acceptance rate on certificate attacks |
| E | Artifact provenance invalidation | Content-addressed builds, sigstores | Applied to (P,S,E,π,Q) package fields with attack suite |
| F | Path to executable guarantees | CompCert, CakeML, WasmCert | Explicit gap docs; not claimed closed in Phase I |

## Occupied — do not hide

- `bv_decide` / LeanSAT LRAT checking already provides bitvector proof certificates.
- Alive2 already does LLVM translation validation.
- CompCert already verifies compilation.

## Strongest currently defensible research claim (Phase I)

> For a restricted pure straight-line bitvector IR (CertIR), we can package programs with
> hash-integrity and independently checked equivalence/functional evidence such that
> ForgeOpt may use intentionally unsound rewrites, yet only semantically equivalent
> cheaper candidates are admitted; certificate/program/spec tampering is rejected by the
> verifier on our adversarial suite (0/7 false accepts after fixing one certificate-quality bug).

## What would make this publishable beyond a framework

- Scale benchmarks + statistical methodology
- Clear TCB and axiom audit (done initially)
- Nontrivial optimizations discovered and generalized
- Closing more of the executable gap **or** precisely characterizing why not
- Effect manifests with rejection of functionally-correct unauthorized programs
