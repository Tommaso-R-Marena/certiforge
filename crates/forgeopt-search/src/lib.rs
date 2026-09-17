//! Untrusted candidate generation for ForgeOpt.
//!
//! Intentionally includes unsound rewrite rules so the independent checker
//! can be demonstrated to reject them.

use certir::{BinOp, Expr, Program};
use forgeopt_cost::program_cost;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone, Debug)]
pub struct Candidate {
    pub program: Program,
    pub transformation: String,
    pub static_cost: u64,
}

#[derive(Clone, Debug)]
pub struct SearchConfig {
    pub seed: u64,
    pub max_candidates: usize,
    pub timeout_ms: u64,
    pub enable_unsound_rules: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            max_candidates: 64,
            timeout_ms: 5_000,
            enable_unsound_rules: true,
        }
    }
}

fn rewrite_expr(expr: &Expr, unsound: bool, default_width: certir::Width) -> Vec<(Expr, String)> {
    let mut out = Vec::new();

    // Local algebraic rewrites (sound on bitvectors when noted)
    match expr {
        Expr::BinOp {
            op: BinOp::Add,
            lhs,
            rhs,
        } => {
            // x + x => x << 1
            if lhs == rhs {
                let w = extract_width_hint(lhs).unwrap_or(default_width);
                out.push((
                    Expr::BinOp {
                        op: BinOp::Shl,
                        lhs: lhs.clone(),
                        rhs: Box::new(Expr::ConstBv {
                            width: w,
                            value: 1,
                        }),
                    },
                    "add_self_to_shl1".into(),
                ));
            }
            // (x&y)+(x^y) => x|y
            if let (
                Expr::BinOp {
                    op: BinOp::And,
                    lhs: a1,
                    rhs: b1,
                },
                Expr::BinOp {
                    op: BinOp::Xor,
                    lhs: a2,
                    rhs: b2,
                },
            ) = (lhs.as_ref(), rhs.as_ref())
            {
                if a1 == a2 && b1 == b2 {
                    out.push((
                        Expr::BinOp {
                            op: BinOp::Or,
                            lhs: a1.clone(),
                            rhs: b1.clone(),
                        },
                        "and_xor_add_to_or".into(),
                    ));
                }
            }
            // commutativity
            out.push((
                Expr::BinOp {
                    op: BinOp::Add,
                    lhs: rhs.clone(),
                    rhs: lhs.clone(),
                },
                "add_commute".into(),
            ));
        }
        Expr::BinOp {
            op: BinOp::Or,
            lhs,
            rhs,
        } => {
            out.push((
                Expr::BinOp {
                    op: BinOp::Or,
                    lhs: rhs.clone(),
                    rhs: lhs.clone(),
                },
                "or_commute".into(),
            ));
            // x | x => x
            if lhs == rhs {
                out.push((*lhs.clone(), "or_idempotent".into()));
            }
        }
        Expr::BinOp {
            op: BinOp::And,
            lhs,
            rhs,
        } => {
            if lhs == rhs {
                out.push((*lhs.clone(), "and_idempotent".into()));
            }
            // x & 0 => 0
            if let Expr::ConstBv { width, value: 0 } = rhs.as_ref() {
                out.push((
                    Expr::ConstBv {
                        width: *width,
                        value: 0,
                    },
                    "and_zero".into(),
                ));
            }
        }
        Expr::BinOp {
            op: BinOp::Xor,
            lhs,
            rhs,
        } => {
            if lhs == rhs {
                let w = extract_width_hint(lhs).unwrap_or(default_width);
                out.push((
                    Expr::ConstBv {
                        width: w,
                        value: 0,
                    },
                    "xor_self".into(),
                ));
            }
        }
        _ => {}
    }

    if unsound {
        // INTENTIONALLY UNSOUND — must be rejected by checker
        if let Expr::BinOp {
            op: BinOp::Add,
            lhs,
            rhs,
        } = expr
        {
            if lhs == rhs {
                out.push((*lhs.clone(), "UNSOUND_add_self_to_self".into()));
            }
            // x + y => x  (unsound)
            out.push((*lhs.clone(), "UNSOUND_add_to_lhs".into()));
        }
        if let Expr::BinOp {
            op: BinOp::Mul,
            lhs,
            ..
        } = expr
        {
            out.push((*lhs.clone(), "UNSOUND_mul_to_lhs".into()));
        }
    }

    // Recurse into children
    match expr {
        Expr::UnOp { op, expr: inner } => {
            for (e, name) in rewrite_expr(inner, unsound, default_width) {
                out.push((
                    Expr::UnOp {
                        op: *op,
                        expr: Box::new(e),
                    },
                    format!("under_unop:{name}"),
                ));
            }
        }
        Expr::BinOp { op, lhs, rhs } => {
            for (e, name) in rewrite_expr(lhs, unsound, default_width) {
                out.push((
                    Expr::BinOp {
                        op: *op,
                        lhs: Box::new(e),
                        rhs: rhs.clone(),
                    },
                    format!("under_lhs:{name}"),
                ));
            }
            for (e, name) in rewrite_expr(rhs, unsound, default_width) {
                out.push((
                    Expr::BinOp {
                        op: *op,
                        lhs: lhs.clone(),
                        rhs: Box::new(e),
                    },
                    format!("under_rhs:{name}"),
                ));
            }
        }
        Expr::Cmp { op, lhs, rhs } => {
            for (e, name) in rewrite_expr(lhs, unsound, default_width) {
                out.push((
                    Expr::Cmp {
                        op: *op,
                        lhs: Box::new(e),
                        rhs: rhs.clone(),
                    },
                    format!("under_cmp_lhs:{name}"),
                ));
            }
            for (e, name) in rewrite_expr(rhs, unsound, default_width) {
                out.push((
                    Expr::Cmp {
                        op: *op,
                        lhs: lhs.clone(),
                        rhs: Box::new(e),
                    },
                    format!("under_cmp_rhs:{name}"),
                ));
            }
        }
        Expr::Select {
            cond,
            then_expr,
            else_expr,
        } => {
            for (e, name) in rewrite_expr(cond, unsound, default_width) {
                out.push((
                    Expr::Select {
                        cond: Box::new(e),
                        then_expr: then_expr.clone(),
                        else_expr: else_expr.clone(),
                    },
                    format!("under_select_cond:{name}"),
                ));
            }
            for (e, name) in rewrite_expr(then_expr, unsound, default_width) {
                out.push((
                    Expr::Select {
                        cond: cond.clone(),
                        then_expr: Box::new(e),
                        else_expr: else_expr.clone(),
                    },
                    format!("under_select_then:{name}"),
                ));
            }
            for (e, name) in rewrite_expr(else_expr, unsound, default_width) {
                out.push((
                    Expr::Select {
                        cond: cond.clone(),
                        then_expr: then_expr.clone(),
                        else_expr: Box::new(e),
                    },
                    format!("under_select_else:{name}"),
                ));
            }
        }
        _ => {}
    }

    out
}

fn extract_width_hint(expr: &Expr) -> Option<certir::Width> {
    match expr {
        Expr::ConstBv { width, .. } => Some(*width),
        Expr::UnOp { expr, .. } => extract_width_hint(expr),
        Expr::BinOp { lhs, .. } => extract_width_hint(lhs),
        Expr::Select { then_expr, .. } => extract_width_hint(then_expr),
        _ => None,
    }
}

fn default_width_of(program: &Program) -> certir::Width {
    match program.ret_ty {
        certir::Ty::BitVec(w) => w,
        certir::Ty::Bool => certir::Width::U8,
    }
}

/// Local rewrite search.
pub fn local_rewrite_search(program: &Program, cfg: &SearchConfig) -> Vec<Candidate> {
    let mut cands = Vec::new();
    let dw = default_width_of(program);
    for (body, transformation) in rewrite_expr(&program.body, cfg.enable_unsound_rules, dw) {
        let mut p = program.clone();
        p.body = body;
        p.name = format!("{}_opt", program.name);
        if p.check().is_ok() {
            let static_cost = program_cost(&p);
            cands.push(Candidate {
                program: p,
                transformation,
                static_cost,
            });
        }
        if cands.len() >= cfg.max_candidates {
            break;
        }
    }
    cands
}

/// Stochastic mutations of operators.
pub fn stochastic_search(program: &Program, cfg: &SearchConfig) -> Vec<Candidate> {
    let mut rng = StdRng::seed_from_u64(cfg.seed);
    let mut cands = Vec::new();
    let ops = [
        BinOp::Add,
        BinOp::Sub,
        BinOp::Or,
        BinOp::And,
        BinOp::Xor,
        BinOp::Mul,
    ];
    for i in 0..cfg.max_candidates.min(32) {
        let mut p = program.clone();
        p.body = mutate_ops(&program.body, &mut rng, &ops);
        p.name = format!("{}_mut{i}", program.name);
        if p.check().is_ok() {
            cands.push(Candidate {
                static_cost: program_cost(&p),
                program: p,
                transformation: format!("stochastic_mut_{i}"),
            });
        }
    }
    cands
}

fn mutate_ops(expr: &Expr, rng: &mut StdRng, ops: &[BinOp]) -> Expr {
    match expr {
        Expr::BinOp { op, lhs, rhs } => {
            let new_op = if rng.gen_bool(0.3) {
                ops[rng.gen_range(0..ops.len())]
            } else {
                *op
            };
            Expr::BinOp {
                op: new_op,
                lhs: Box::new(mutate_ops(lhs, rng, ops)),
                rhs: Box::new(mutate_ops(rhs, rng, ops)),
            }
        }
        Expr::UnOp { op, expr } => Expr::UnOp {
            op: *op,
            expr: Box::new(mutate_ops(expr, rng, ops)),
        }
        other => other.clone(),
    }
}

pub fn combined_search(program: &Program, cfg: &SearchConfig) -> Vec<Candidate> {
    let mut all = local_rewrite_search(program, cfg);
    all.extend(stochastic_search(program, cfg));
    all.sort_by_key(|c| c.static_cost);
    all.truncate(cfg.max_candidates);
    all
}

#[cfg(test)]
mod tests {
    use super::*;
    use certir_parser::parse_program;

    #[test]
    fn finds_or_rewrite() {
        let p = parse_program(
            "fn p(x: u32, y: u32) -> u32 { add(and(x, y), xor(x, y)) }",
        )
        .unwrap();
        let cfg = SearchConfig {
            enable_unsound_rules: false,
            ..Default::default()
        };
        let cands = local_rewrite_search(&p, &cfg);
        assert!(
            cands.iter().any(|c| c.transformation.contains("and_xor_add_to_or")),
            "expected and_xor_add_to_or candidate, got {:?}",
            cands.iter().map(|c| &c.transformation).collect::<Vec<_>>()
        );
    }

    #[test]
    fn emits_unsound_candidates() {
        let p = parse_program("fn p(x: u32) -> u32 { add(x, x) }").unwrap();
        let cfg = SearchConfig {
            enable_unsound_rules: true,
            ..Default::default()
        };
        let cands = local_rewrite_search(&p, &cfg);
        assert!(cands.iter().any(|c| c.transformation.contains("UNSOUND")));
    }
}
