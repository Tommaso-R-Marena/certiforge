# Specification Gap

A proof that `P` satisfies specification `S` is **not** a proof that `S` matches the
user's informal intent.

CERTIFORGE records specification provenance (`human` | `ai-proposed` | …) and rejects
obviously vacuous preconditions (`Pre = False`). These audits are diagnostics, not a
complete solution to intent verification.

Natural-language → formal spec verification is a separate research problem and is
**out of scope** for Phase I.
