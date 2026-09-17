# Failed Claims

## Certificate quality optional on exhaustive domains (withdrawn)

**Claimed implicitly by implementation:** For u8/u16, interpreter exhaustive checks alone
suffice for ACCEPT even with empty/corrupt Lean certificate files (if hashes match).

**Status:** FALSE / withdrawn. Adversarial testing produced a false accept.

**Repair:** Substantive certificate requirement is mandatory whenever the manifest claims
functional/equivalence certificates.
