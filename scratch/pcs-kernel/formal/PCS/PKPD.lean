import PCS.Core

namespace PCS.PKPD

inductive Dimension
  | mass
  | volume
  | time
  | concentration
  | volumePerTime
  deriving DecidableEq, Repr

inductive Unit
  | mg | g
  | mL | L
  | min | h
  | mgPerL | gPerL
  | mLPerMin | LPerH
  deriving DecidableEq, Repr

def dimension : Unit → Dimension
  | Unit.mg | Unit.g => Dimension.mass
  | Unit.mL | Unit.L => Dimension.volume
  | Unit.min | Unit.h => Dimension.time
  | Unit.mgPerL | Unit.gPerL => Dimension.concentration
  | Unit.mLPerMin | Unit.LPerH => Dimension.volumePerTime

def Compatible (u v : Unit) : Prop := dimension u = dimension v

structure OneCompartmentIVContract where
  doseUnit : Unit
  volumeUnit : Unit
  clearanceUnit : Unit
  timeUnit : Unit
  concentrationUnit : Unit
  doseIsMass : dimension doseUnit = Dimension.mass
  volumeIsVolume : dimension volumeUnit = Dimension.volume
  clearanceIsVolumePerTime : dimension clearanceUnit = Dimension.volumePerTime
  timeIsTime : dimension timeUnit = Dimension.time
  concentrationIsMassPerVolume : dimension concentrationUnit = Dimension.concentration

example : Compatible Unit.mgPerL Unit.gPerL := by
  rfl

example : Compatible Unit.h Unit.min := by
  rfl

def canonicalContract : OneCompartmentIVContract where
  doseUnit := Unit.mg
  volumeUnit := Unit.L
  clearanceUnit := Unit.LPerH
  timeUnit := Unit.h
  concentrationUnit := Unit.mgPerL
  doseIsMass := rfl
  volumeIsVolume := rfl
  clearanceIsVolumePerTime := rfl
  timeIsTime := rfl
  concentrationIsMassPerVolume := rfl

theorem canonical_contract_units_valid :
    dimension canonicalContract.doseUnit = Dimension.mass ∧
    dimension canonicalContract.volumeUnit = Dimension.volume ∧
    dimension canonicalContract.clearanceUnit = Dimension.volumePerTime ∧
    dimension canonicalContract.timeUnit = Dimension.time ∧
    dimension canonicalContract.concentrationUnit = Dimension.concentration := by
  exact ⟨canonicalContract.doseIsMass,
    canonicalContract.volumeIsVolume,
    canonicalContract.clearanceIsVolumePerTime,
    canonicalContract.timeIsTime,
    canonicalContract.concentrationIsMassPerVolume⟩

end PCS.PKPD
