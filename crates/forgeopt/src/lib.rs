//! ForgeOpt — untrusted superoptimizer admitted only by independent checking.

use certiforge_package::check_equivalence;
use certir::Program;
use forgeopt_cost::program_cost;
use forgeopt_search::{combined_search, Candidate, SearchConfig};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct OptimizeResult {
    pub original_cost: u64,
    pub best: Option<AdmittedCandidate>,
    pub rejected: Vec<RejectedCandidate>,
    pub candidates_considered: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct AdmittedCandidate {
    pub program: Program,
    pub transformation: String,
    pub static_cost: u64,
    pub improvement: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct RejectedCandidate {
    pub transformation: String,
    pub static_cost: u64,
    pub reason: String,
}

/// Run untrusted search; admit a candidate only if equivalence checking succeeds
/// and static cost strictly improves.
pub fn optimize(program: &Program, cfg: SearchConfig) -> OptimizeResult {
    let original_cost = program_cost(program);
    let candidates = combined_search(program, &cfg);
    let mut rejected = Vec::new();
    let mut best: Option<AdmittedCandidate> = None;

    for cand in &candidates {
        if cand.static_cost >= original_cost {
            rejected.push(RejectedCandidate {
                transformation: cand.transformation.clone(),
                static_cost: cand.static_cost,
                reason: "no static cost improvement".into(),
            });
            continue;
        }
        match check_equivalence(program, &cand.program, cfg.seed) {
            Ok(()) => {
                let improvement = original_cost - cand.static_cost;
                let admit = AdmittedCandidate {
                    program: cand.program.clone(),
                    transformation: cand.transformation.clone(),
                    static_cost: cand.static_cost,
                    improvement,
                };
                match &best {
                    None => best = Some(admit),
                    Some(cur) if admit.static_cost < cur.static_cost => best = Some(admit),
                    _ => {}
                }
            }
            Err(reason) => rejected.push(RejectedCandidate {
                transformation: cand.transformation.clone(),
                static_cost: cand.static_cost,
                reason,
            }),
        }
    }

    OptimizeResult {
        original_cost,
        best,
        rejected,
        candidates_considered: candidates.len(),
    }
}

/// Expose candidate enumeration for adversarial tests.
pub fn generate_candidates(program: &Program, cfg: SearchConfig) -> Vec<Candidate> {
    combined_search(program, &cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use certir_parser::parse_program;
    use forgeopt_search::SearchConfig;

    #[test]
    fn admits_or_rewrite_rejects_unsound() {
        let p = parse_program("fn p(x: u8, y: u8) -> u8 { add(and(x, y), xor(x, y)) }").unwrap();
        let result = optimize(
            &p,
            SearchConfig {
                enable_unsound_rules: true,
                max_candidates: 128,
                seed: 7,
                ..Default::default()
            },
        );
        assert!(
            result.best.is_some(),
            "expected an admitted optimization, rejected={:?}",
            result.rejected
        );
        let best = result.best.unwrap();
        assert!(best.improvement > 0);
        assert!(result
            .rejected
            .iter()
            .any(|r| r.transformation.contains("UNSOUND") || r.reason.contains("equivalence")));
    }
}
