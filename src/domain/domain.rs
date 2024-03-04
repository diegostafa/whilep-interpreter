use crate::abstract_semantics::state::*;
use crate::domain::expression_tree::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use trait_set::trait_set;

trait_set! {
    pub trait Arithmetic = Sized + Add<Self, Output = Self> +Sub<Self, Output = Self> +Mul<Self, Output = Self> +Div<Self, Output = Self>;
    pub trait DomainProperties = Sized + Display + Clone + Copy + Eq + Debug;
}

pub trait Domain: DomainProperties + Lattice + Arithmetic {
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>);
    fn eval_specific_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self>;

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
            ArithmeticExpr::Variable(var) => (state.read(var), state.clone()),
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
            BooleanExpr::Not(b) => Self::eval_bexpr(&negate_bexpr(b), state),
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
            BooleanExpr::NumEq(a1, a2) => {
                let (ltree, _) = ExpressionTree::build(a1, state);
                let (rtree, _) = ExpressionTree::build(a2, state);
                let (i1, i2) = (ltree.get_value(), rtree.get_value());
                let intersection = i1.intersection(&i2);

                match intersection == Self::BOT {
                    true => State::Bottom,
                    _ => {
                        let new_state = state
                            .refine_expression_tree(&ltree, intersection)
                            .refine_expression_tree(&rtree, intersection);

                        let a1_state = Self::eval_aexpr(a1, &new_state).1;
                        let a2_state = Self::eval_aexpr(a2, &a1_state).1;
                        a2_state
                    }
                }
            }
            BooleanExpr::NumNotEq(a1, a2) => match a1 == a2 {
                true => State::Bottom,
                _ => {
                    let (_, new_state) = Self::eval_aexpr(a1, state);
                    let (_, new_state) = Self::eval_aexpr(a2, &new_state);
                    new_state
                }
            },

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

            BooleanExpr::NumLt(a1, a2) => match a1 == a2 {
                true => State::Bottom,
                _ => Self::eval_specific_bexpr(expr, state),
            },
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
