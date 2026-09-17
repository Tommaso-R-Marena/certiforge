//! Deterministic certificate package format and independent verifier.
//!
//! Acceptance requires:
//! - canonical hashes of program / optimized / spec / certificates match manifest
//! - programs are well-typed
//! - functional certificate present and checkable
//! - equivalence certificate present and checkable
//! - vacuity audit does not flag an impossible precondition (diagnostic reject)
//!
//! Phase I equivalence/functional certificates are checked by:
//! 1. Structural presence + hash integrity (always)
//! 2. Exhaustive/SMT-style bitvector equivalence via the interpreter on a
//!    complete domain for narrow widths, or random+edge sampling plus an
//!    explicit Lean theorem artifact when provided
//! 3. Optional Lean `lake env lean` check of bundled `.lean` certificates
//!
//! TRUST: the Rust checker is part of the executable TCB for Phase I.
//! The Lean soundness theorem covers the abstract model; refinement to this
//! checker is differential-tested, not yet proved.

use certir::{Program, Ty, Width};
use certir_interpreter::{eval, observationally_equal, Value};
use certir_parser::{parse_program, pretty_program};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
