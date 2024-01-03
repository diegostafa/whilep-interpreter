use crate::ast::*;
use crate::state::*;

pub fn statement_expr(sexpr: &StatementExpr, state: &mut State) {
    match sexpr {
        StatementExpr::Skip => {}

        StatementExpr::Chain(s1, s2) => {
            statement_expr(s1, state);
            statement_expr(s2, state);
        }

        StatementExpr::Assignment { var, val } => {
            state.insert(var.to_string(), arithmetic_expr(val, state));
        }

        StatementExpr::Conditional {
            cond,
            tt_branch,
            ff_branch,
        } => {
            if boolean_expr(cond, state) {
                statement_expr(tt_branch, state)
            } else {
                statement_expr(ff_branch, state)
            }
        }

        StatementExpr::WhileLoop { cond, body } => {
            while boolean_expr(cond, state) {
                statement_expr(body, state);
            }
        }
    }
}

pub fn arithmetic_expr(expr: &ArithmeticExpr, state: &State) -> i32 {
    match expr {
        ArithmeticExpr::Number(n) => *n,
        ArithmeticExpr::Identifier(var) => state.get(var).unwrap().to_owned(),
        ArithmeticExpr::Add(a1, a2) => arithmetic_expr(a1, state) + arithmetic_expr(a2, state),
        ArithmeticExpr::Sub(a1, a2) => arithmetic_expr(a1, state) - arithmetic_expr(a2, state),
        ArithmeticExpr::Mul(a1, a2) => arithmetic_expr(a1, state) * arithmetic_expr(a2, state),
        ArithmeticExpr::Div(a1, a2) => arithmetic_expr(a1, state) / arithmetic_expr(a2, state),
    }
}

pub fn boolean_expr(expr: &BooleanExpr, state: &State) -> bool {
    match expr {
        BooleanExpr::True => true,
        BooleanExpr::False => false,
        BooleanExpr::Not(b) => !boolean_expr(b, state),
        BooleanExpr::And(b1, b2) => boolean_expr(b1, state) && boolean_expr(b2, state),
        BooleanExpr::NumEq(a1, a2) => arithmetic_expr(a1, state) == arithmetic_expr(a2, state),
        BooleanExpr::NumLtEq(a1, a2) => arithmetic_expr(a1, state) <= arithmetic_expr(a2, state),
    }
}
