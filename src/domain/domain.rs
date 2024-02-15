use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use trait_set::trait_set;

use crate::abstract_semantics::state::*;
use crate::domain::expression_tree::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

trait_set! {
    pub trait Arithmetic = Sized + Add<Self, Output = Self> +Sub<Self, Output = Self> +Mul<Self, Output = Self> +Div<Self, Output = Self>;
    pub trait DomainProperties = Sized + Lattice + Display + Clone + Copy + Eq + Arithmetic + Debug;
}

pub trait Domain: DomainProperties {
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>);
    fn eval_specific_bexpr(cmp_expr: &BooleanExpr, state: &State<Self>) -> State<Self>;

    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        match expr {
            ArithmeticExpr::Number(_) | ArithmeticExpr::Interval(_, _) => {
                Self::eval_specific_aexpr(expr, state)
            }

            ArithmeticExpr::PostIncrement(var) => {
                let val = state.read(var);
                (val, state.put(var, val + Self::UNIT))
            }
            ArithmeticExpr::PostDecrement(var) => {
                let val = state.read(var);
                (val, state.put(var, val - Self::UNIT))
            }

            ArithmeticExpr::Identifier(var) => (state.read(var), state.clone()),
            ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
            ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
            ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
            ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),
        }
    }

    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match expr {
            BooleanExpr::True => state.clone(),
            BooleanExpr::False => State::Bottom,
            BooleanExpr::Not(b) => Self::eval_bexpr(&desugar_not_bexpr(*b.clone()), state),
            BooleanExpr::And(b1, b2) => {
                let lhs1 = Self::eval_bexpr(b1, state);
                let lhs2 = Self::eval_bexpr(b2, &lhs1);
                let rhs2 = Self::eval_bexpr(b2, state);
                let rhs1 = Self::eval_bexpr(b1, &rhs2);
                lhs2.intersection(&rhs1)
            }
            BooleanExpr::Or(b1, b2) => {
                let lhs1 = Self::eval_bexpr(b1, state);
                let lhs2 = Self::eval_bexpr(b2, &lhs1);
                let rhs2 = Self::eval_bexpr(b2, state);
                let rhs1 = Self::eval_bexpr(b1, &rhs2);
                lhs2.union(&rhs1)
            }
            BooleanExpr::NumLtEq(a1, a2) => {
                let lt = Self::eval_bexpr(&BooleanExpr::NumLt(a1.clone(), a2.clone()), state);
                let eq = Self::eval_bexpr(&BooleanExpr::NumEq(a1.clone(), a2.clone()), state);
                lt.union(&eq)
            }

            BooleanExpr::NumGt(a1, a2) => {
                Self::eval_bexpr(&BooleanExpr::NumLt(a2.clone(), a1.clone()), state)
            }
            BooleanExpr::NumGtEq(a1, a2) => {
                Self::eval_bexpr(&BooleanExpr::NumLtEq(a2.clone(), a1.clone()), state)
            }

            BooleanExpr::NumEq(_, _) | BooleanExpr::NumNotEq(_, _) | BooleanExpr::NumLt(_, _) => {
                Self::eval_specific_bexpr(expr, state)
            }
        }
    }

    fn build_expression_tree(
        expr: &ArithmeticExpr,
        state: &State<Self>,
    ) -> (ExpressionTree<Self>, State<Self>) {
        let (val, new_state) = Self::eval_aexpr(expr, state);

        match expr {
            ArithmeticExpr::Number(_) | ArithmeticExpr::Interval(_, _) => {
                (ExpressionTree::Value(val), new_state)
            }

            ArithmeticExpr::Identifier(var)
            | ArithmeticExpr::PostIncrement(var)
            | ArithmeticExpr::PostDecrement(var) => {
                (ExpressionTree::Variable(var.to_string(), val), new_state)
            }

            ArithmeticExpr::Add(a1, a2)
            | ArithmeticExpr::Sub(a1, a2)
            | ArithmeticExpr::Mul(a1, a2)
            | ArithmeticExpr::Div(a1, a2) => {
                let (l, new_state) = Self::build_expression_tree(a1, &new_state);
                let (r, new_state) = Self::build_expression_tree(a2, &new_state);

                (
                    ExpressionTree::Binop(expr.clone(), val, Box::new(l), Box::new(r)),
                    new_state,
                )
            }
        }
    }
}

pub fn binop_aexpr<T: Domain>(
    op: fn(T, T) -> T,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> (T, State<T>) {
    let (i1, new_state) = T::eval_aexpr(a1, &state);
    let (i2, new_state) = T::eval_aexpr(a2, &new_state);
    (op(i1, i2), new_state)
}

pub fn binop_cmp<T: Domain>(
    op: fn(T, T) -> bool,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> State<T> {
    let (i1, new_state) = T::eval_aexpr(a1, &state);
    let (i2, new_state) = T::eval_aexpr(a2, &new_state);
    match op(i1, i2) {
        true => new_state,
        _ => State::Bottom,
    }
}
