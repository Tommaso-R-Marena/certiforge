# Formal Model (Phase I)

Canonical definitions live in `formal/CertiForge/`.

- `eval : Program → List Value → Option Value`
- `SemEq P Q := ∀ inputs, eval P inputs = eval Q inputs`
- `Satisfies P S := ∀ inputs, S.pre inputs → ∃ y, eval P inputs = some y ∧ S.post inputs y`
- `PackageSemanticallyValid` includes hash consistency flags, SemEq, and Satisfies for P and Q

Effects: stubbed (`Effects.lean`); Phase I traces are empty.
