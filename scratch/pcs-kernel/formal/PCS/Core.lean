namespace PCS

inductive ClaimKind
  | formal
  | computational
  | empirical
  | mixed
  deriving DecidableEq, Repr

inductive EvidenceKind
  | formalProof
  | computationalTest
  | statisticalValidation
  | empiricalValidation
  | provenance
  deriving DecidableEq, Repr

inductive Outcome
  | pass
  | fail
  | unverified
  deriving DecidableEq, Repr

inductive Predicate
  | opaque (tag : String)
  | csvDisjoint (leftArtifact rightArtifact key : String)
  | reactionBalanced (signature : String)
  | unitsCompatible (leftUnit rightUnit : String)
  | pkpdContract (modelArtifact : String)
  | pkpdReferenceMatch (modelArtifact outputArtifact : String)
  deriving DecidableEq, Repr

structure Evidence where
  id : String
  kind : EvidenceKind
  outcome : Outcome
  predicate : Option Predicate := none
  deriving DecidableEq, Repr

structure Claim where
  id : String
  kind : ClaimKind
  predicate : Option Predicate := none
  requiredEvidence : List String := []
  assumptions : List String := []
  deriving DecidableEq, Repr

def BoundTo (e : Evidence) (c : Claim) : Prop :=
  e.id ∈ c.requiredEvidence ∧ e.predicate = c.predicate

inductive AssuranceLevel
  | formal
  | computational
  | empirical
  | mixed
  deriving DecidableEq, Repr

inductive Assures : AssuranceLevel → Claim → List Evidence → Prop
  | formal {c : Claim} {es : List Evidence} (e : Evidence)
      (member : e ∈ es)
      (claimKind : c.kind = ClaimKind.formal)
      (evidenceKind : e.kind = EvidenceKind.formalProof)
      (passed : e.outcome = Outcome.pass)
      (bound : BoundTo e c) : Assures AssuranceLevel.formal c es
  | computational {c : Claim} {es : List Evidence} (e : Evidence)
      (member : e ∈ es)
      (claimKind : c.kind = ClaimKind.computational)
      (evidenceKind : e.kind = EvidenceKind.computationalTest)
      (passed : e.outcome = Outcome.pass)
      (bound : BoundTo e c) : Assures AssuranceLevel.computational c es
  | empirical {c : Claim} {es : List Evidence} (e : Evidence)
      (member : e ∈ es)
      (claimKind : c.kind = ClaimKind.empirical)
      (evidenceKind : e.kind = EvidenceKind.empiricalValidation ∨
                      e.kind = EvidenceKind.statisticalValidation)
      (passed : e.outcome = Outcome.pass)
      (bound : BoundTo e c) : Assures AssuranceLevel.empirical c es
  | mixed {c : Claim} {es : List Evidence} (correctness empiricalEvidence : Evidence)
      (correctnessMember : correctness ∈ es)
      (empiricalMember : empiricalEvidence ∈ es)
      (claimKind : c.kind = ClaimKind.mixed)
      (correctnessKind : correctness.kind = EvidenceKind.formalProof)
      (empiricalKind : empiricalEvidence.kind = EvidenceKind.empiricalValidation ∨
                       empiricalEvidence.kind = EvidenceKind.statisticalValidation)
      (correctnessPassed : correctness.outcome = Outcome.pass)
      (empiricalPassed : empiricalEvidence.outcome = Outcome.pass)
      (correctnessBound : BoundTo correctness c)
      (empiricalBound : BoundTo empiricalEvidence c) : Assures AssuranceLevel.mixed c es

theorem formal_assurance_has_checked_formal_proof
    {c : Claim} {es : List Evidence} (h : Assures AssuranceLevel.formal c es) :
    ∃ e, e ∈ es ∧ e.kind = EvidenceKind.formalProof ∧ e.outcome = Outcome.pass := by
  cases h with
  | formal e member _ evidenceKind passed _ =>
      exact ⟨e, member, evidenceKind, passed⟩

theorem computational_assurance_is_predicate_bound
    {c : Claim} {es : List Evidence} (h : Assures AssuranceLevel.computational c es) :
    ∃ e, e ∈ es ∧ e.id ∈ c.requiredEvidence ∧ e.predicate = c.predicate := by
  cases h with
  | computational e member _ _ _ bound =>
      exact ⟨e, member, bound.1, bound.2⟩

theorem computational_assurance_cannot_use_unverified
    {c : Claim} {es : List Evidence} (h : Assures AssuranceLevel.computational c es) :
    ∃ e, e ∈ es ∧ e.outcome = Outcome.pass := by
  cases h with
  | computational e member _ _ passed _ =>
      exact ⟨e, member, passed⟩

theorem empirical_assurance_requires_empirical_class
    {c : Claim} {es : List Evidence} (h : Assures AssuranceLevel.empirical c es) :
    ∃ e, e ∈ es ∧
      (e.kind = EvidenceKind.empiricalValidation ∨ e.kind = EvidenceKind.statisticalValidation) ∧
      e.outcome = Outcome.pass := by
  cases h with
  | empirical e member _ evidenceKind passed _ =>
      exact ⟨e, member, evidenceKind, passed⟩

theorem mixed_assurance_requires_both_classes
    {c : Claim} {es : List Evidence} (h : Assures AssuranceLevel.mixed c es) :
    ∃ correctness empiricalEvidence,
      correctness ∈ es ∧ empiricalEvidence ∈ es ∧
      correctness.kind = EvidenceKind.formalProof ∧
      (empiricalEvidence.kind = EvidenceKind.empiricalValidation ∨
       empiricalEvidence.kind = EvidenceKind.statisticalValidation) ∧
      correctness.outcome = Outcome.pass ∧ empiricalEvidence.outcome = Outcome.pass := by
  cases h with
  | mixed correctness empiricalEvidence cm em _ ck ek cp ep _ _ =>
      exact ⟨correctness, empiricalEvidence, cm, em, ck, ek, cp, ep⟩

end PCS
