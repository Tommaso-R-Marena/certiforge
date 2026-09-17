/-
  Effects module — Phase I stub.
  Abstract effect traces are introduced in Phase IV.
  This file exists so the architecture documents a deliberate boundary:
  Phase I programs are pure; no effect traces are produced.
-/
import CertiForge.Syntax

namespace CertiForge

/-- Effect constructors (Phase IV). Not used by Phase I packages. -/
inductive Effect where
  | read    (resource : String)
  | write   (resource : String)
  | spawn   (program : String)
  | network (endpoint : String)
  | emit    (channel : String) (dataClass : String)
  deriving DecidableEq, Repr

/-- An effect manifest lists authorized effects. -/
structure EffectManifest where
  allowed : List Effect
  deriving Repr

/-- Every effect in a trace is authorized. -/
def Allowed (E : EffectManifest) (trace : List Effect) : Prop :=
  ∀ e ∈ trace, e ∈ E.allowed

/-- Phase I programs produce an empty effect trace. -/
def pureTrace : List Effect := []

theorem pure_allowed (E : EffectManifest) : Allowed E pureTrace := by
  intro e h
  cases h

end CertiForge
