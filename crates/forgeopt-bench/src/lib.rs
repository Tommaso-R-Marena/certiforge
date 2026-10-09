//! Benchmark harness — empirical only; never treated as proof.

use certir_interpreter::{eval, Value};
use certir_parser::parse_program;
use forgeopt::optimize;
use forgeopt_cost::program_cost;
use forgeopt_search::SearchConfig;
use serde::Serialize;
use std::time::Instant;

#[derive(Clone, Debug, Serialize)]
pub struct BenchCaseResult {
    pub name: String,
    pub original_cost: u64,
    pub optimized_cost: Option<u64>,
    pub improvement: Option<u64>,
    pub transformation: Option<String>,
    pub admitted: bool,
    pub search_ms: u128,
    pub verify_note: String,
}

pub fn run_smoke_suite() -> Vec<BenchCaseResult> {
    let cases = [
        (
            "and_xor_add_u8",
            "fn p(x: u8, y: u8) -> u8 { add(and(x, y), xor(x, y)) }",
        ),
        ("add_self_u16", "fn p(x: u16) -> u16 { add(x, x) }"),
        ("xor_self_u8", "fn p(x: u8) -> u8 { xor(x, x) }"),
        ("and_zero_u8", "fn p(x: u8) -> u8 { and(x, u8(0)) }"),
    ];

    let mut results = Vec::new();
    for (name, src) in cases {
        let p = parse_program(src).expect("parse");
        let t0 = Instant::now();
        let opt = optimize(
            &p,
            SearchConfig {
                seed: 42,
                max_candidates: 64,
                enable_unsound_rules: true,
                timeout_ms: 2000,
            },
        );
        let search_ms = t0.elapsed().as_millis();
        results.push(BenchCaseResult {
            name: name.into(),
            original_cost: opt.original_cost,
            optimized_cost: opt.best.as_ref().map(|b| b.static_cost),
            improvement: opt.best.as_ref().map(|b| b.improvement),
            transformation: opt.best.as_ref().map(|b| b.transformation.clone()),
            admitted: opt.best.is_some(),
            search_ms,
            verify_note: format!(
                "rejected {} unsound/non-improving candidates",
                opt.rejected.len()
            ),
        });

        // Empirical micro-timing (not a proof of speedup)
        if let Some(best) = opt.best {
            let _ = program_cost(&best.program);
            let inputs = [Value::bitvec(certir::Width::U8, 7)];
            // Only run if arity matches
            if p.params.len() == 1 {
                let _ = eval(&p, &inputs);
                let _ = eval(&best.program, &inputs);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_finds_at_least_one_opt() {
        let r = run_smoke_suite();
        assert!(r.iter().any(|c| c.admitted), "{r:?}");
    }
}
