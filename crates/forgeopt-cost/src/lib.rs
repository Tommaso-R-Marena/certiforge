use certir::{BinOp, Expr, Program};

/// Weighted static cost: cheaper ops weigh less.
pub fn op_weight(op: BinOp) -> u64 {
    match op {
        BinOp::And | BinOp::Or | BinOp::Xor => 1,
        BinOp::Shl | BinOp::Lshr => 1,
        BinOp::Add | BinOp::Sub => 2,
        BinOp::Mul => 4,
    }
}

pub fn expr_cost(expr: &Expr) -> u64 {
    match expr {
        Expr::ConstBool { .. } | Expr::ConstBv { .. } | Expr::Var { .. } => 0,
        Expr::UnOp { expr, .. } => 1 + expr_cost(expr),
        Expr::BinOp { op, lhs, rhs } => op_weight(*op) + expr_cost(lhs) + expr_cost(rhs),
        Expr::Cmp { lhs, rhs, .. } => 2 + expr_cost(lhs) + expr_cost(rhs),
        Expr::Select {
            cond,
            then_expr,
            else_expr,
        } => 3 + expr_cost(cond) + expr_cost(then_expr) + expr_cost(else_expr),
    }
}

pub fn program_cost(p: &Program) -> u64 {
    expr_cost(&p.body)
}

pub fn node_count(p: &Program) -> usize {
    p.body.node_count()
}
