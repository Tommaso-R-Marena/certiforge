//! Differential and mutation fuzzing utilities.

use certiforge_mutate::generate_mutants;
use certir::{BinOp, Expr, Param, Program, Ty, Width};
use certir_interpreter::{observationally_equal, Value};
use rand::{Rng, SeedableRng};

pub fn random_expr<R: Rng>(rng: &mut R, width: Width, depth: u32) -> Expr {
    if depth == 0 || rng.gen_bool(0.3) {
        if rng.gen_bool(0.5) {
            Expr::Var { name: "x".into() }
        } else {
            Expr::ConstBv {
                width,
                value: rng.gen::<u64>() & width.mask(),
            }
        }
    } else {
        let ops = [BinOp::Add, BinOp::And, BinOp::Or, BinOp::Xor, BinOp::Sub];
        Expr::BinOp {
            op: ops[rng.gen_range(0..ops.len())],
            lhs: Box::new(random_expr(rng, width, depth - 1)),
            rhs: Box::new(random_expr(rng, width, depth - 1)),
        }
    }
}

pub fn random_program(seed: u64) -> Program {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let width = Width::U8;
    Program {
        name: format!("rand_{seed}"),
        params: vec![Param {
            name: "x".into(),
            ty: Ty::BitVec(width),
        }],
        body: random_expr(&mut rng, width, 3),
        ret_ty: Ty::BitVec(width),
    }
}

/// Mutants of a correct program should rarely be observationally equal.
pub fn mutation_rejection_rate(program: &Program, n: usize, seed: u64) -> f64 {
    let mutants = generate_mutants(program, n, seed);
    let samples: Vec<Vec<Value>> = (0..32)
        .map(|i| vec![Value::bitvec(Width::U8, i as u64)])
        .collect();
    let mut rejected = 0usize;
    let mut considered = 0usize;
    for m in mutants {
        if m.check().is_err() {
            rejected += 1;
            considered += 1;
            continue;
        }
        considered += 1;
        let eq = observationally_equal(program, &m, &samples).unwrap_or(false);
        if !eq {
            rejected += 1;
        }
    }
    if considered == 0 {
        1.0
    } else {
        rejected as f64 / considered as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use certir_interpreter::eval;
    use certir_parser::parse_program;

    #[test]
    fn random_programs_typecheck_often() {
        let mut ok = 0;
        for s in 0..20 {
            let p = random_program(s);
            if p.check().is_ok() {
                let _ = eval(&p, &[Value::bitvec(Width::U8, 3)]);
                ok += 1;
            }
        }
        assert!(ok > 0);
    }

    #[test]
    fn mutants_usually_rejected() {
        let p = parse_program("fn p(x: u8) -> u8 { add(x, u8(1)) }").unwrap();
        let rate = mutation_rejection_rate(&p, 40, 9);
        assert!(rate > 0.5, "rejection rate {rate}");
    }
}
