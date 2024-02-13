use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use trait_set::trait_set;

use crate::abstract_semantics::state::*;
use crate::domain::expression_tree::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

trait_set! {
    pub trait Arithmetic = Sized + Add<Self, Output = Self> +Sub<Self, Output = Self> +Mul<Self, Output = Self> +Div<Self, Output = Self>;
    pub trait DomainProperties = Sized + Lattice + Display + Clone + Eq + Arithmetic + Debug;
}

pub trait Domain: DomainProperties {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>);
    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self>;

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
