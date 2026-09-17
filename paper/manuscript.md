# CERTIFORGE — Manuscript Outline

## Thesis (provisional)

Untrusted AI-generated programs can be packaged with independently checkable functional
guarantees for a restricted IR, while an untrusted superoptimizer improves static cost
without entering the trusted computing base — provided equivalence certificates are
checked independently and false acceptance is measured adversarially.

## Sections (draft)

1. Introduction and trust philosophy
2. Related work (PCC, TV, CompCert, Alive2, superopt, LLM+FM)
3. CertIR and formal model
4. Certificate packages and checker soundness
5. ForgeOpt: untrusted search, trusted admission
6. Evaluation: optimization, timing, false acceptance
7. Limitations: specification gap, executable gap, axioms
8. Conclusion

## Theorems appendix

See `formal/` and `paper/theorem_appendix.md`.
