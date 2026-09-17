//! Program mutation operators for adversarial evaluation.

use certir::{BinOp, Expr, Program};
use rand::Rng;

pub fn mutate_constants(expr: &Expr, delta: i64) -> Expr {
    match expr {
        Expr::ConstBv { width, value } => {
            let v = if delta >= 0 {
                value.wrapping_add(delta as u64)
            } else {
                value.wrapping_sub((-delta) as u64)
            };
            Expr::ConstBv {
                width: *width,
                value: v & width.mask(),
            }
        }
        Expr::UnOp { op, expr } => Expr::UnOp {
            op: *op,
            expr: Box::new(mutate_constants(expr, delta)),
        },
        Expr::BinOp { op, lhs, rhs } => Expr::BinOp {
            op: *op,
            lhs: Box::new(mutate_constants(lhs, delta)),
            rhs: Box::new(mutate_constants(rhs, delta)),
        },
        Expr::Cmp { op, lhs, rhs } => Expr::Cmp {
            op: *op,
            lhs: Box::new(mutate_constants(lhs, delta)),
            rhs: Box::new(mutate_constants(rhs, delta)),
        },
        Expr::Select {
            cond,
            then_expr,
            else_expr,
        } => Expr::Select {
            cond: Box::new(mutate_constants(cond, delta)),
            then_expr: Box::new(mutate_constants(then_expr, delta)),
            else_expr: Box::new(mutate_constants(else_expr, delta)),
        },
        other => other.clone(),
    }
}

pub fn swap_add_mul(expr: &Expr) -> Expr {
    match expr {
        Expr::BinOp {
            op: BinOp::Add,
            lhs,
            rhs,
        } => Expr::BinOp {
            op: BinOp::Mul,
            lhs: Box::new(swap_add_mul(lhs)),
            rhs: Box::new(swap_add_mul(rhs)),
        },
        Expr::BinOp { op, lhs, rhs } => Expr::BinOp {
            op: *op,
            lhs: Box::new(swap_add_mul(lhs)),
            rhs: Box::new(swap_add_mul(rhs)),
        },
        Expr::UnOp { op, expr } => Expr::UnOp {
            op: *op,
            expr: Box::new(swap_add_mul(expr)),
        },
        Expr::Cmp { op, lhs, rhs } => Expr::Cmp {
            op: *op,
            lhs: Box::new(swap_add_mul(lhs)),
            rhs: Box::new(swap_add_mul(rhs)),
        },
        Expr::Select {
            cond,
            then_expr,
            else_expr,
        } => Expr::Select {
            cond: Box::new(swap_add_mul(cond)),
            then_expr: Box::new(swap_add_mul(then_expr)),
            else_expr: Box::new(swap_add_mul(else_expr)),
        },
        other => other.clone(),
    }
}

pub fn mutate_program<R: Rng>(program: &Program, rng: &mut R) -> Program {
    let mut p = program.clone();
    p.name = format!("{}_mut", program.name);
    if rng.gen_bool(0.5) {
        p.body = mutate_constants(&program.body, if rng.gen_bool(0.5) { 1 } else { -1 });
    } else {
        p.body = swap_add_mul(&program.body);
    }
    p
}

pub fn generate_mutants(program: &Program, n: usize, seed: u64) -> Vec<Program> {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..n).map(|_| mutate_program(program, &mut rng)).collect()
}
