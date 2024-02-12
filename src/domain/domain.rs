use std::fmt::Display;
use trait_set::trait_set;

use crate::abstract_semantics::state::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

trait_set! {
    pub trait DomainProperties = Sized + Lattice + Display + Clone + Eq;
}

pub trait Domain: DomainProperties {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>);
    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self>;
}

pub fn trans_aexpr<T: Domain>(
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> (T, T, State<T>) {
    let (i1, new_state) = T::eval_aexpr(a1, &state);
    let (i2, new_state) = T::eval_aexpr(a2, &new_state);
    (i1, i2, new_state)
}

pub fn binop_aexpr<T: Domain>(
    op: fn(T, T) -> T,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> (T, State<T>) {
    let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
    (op(i1, i2), new_state)
}

pub fn binop_cmp<T: Domain>(
    op: fn(T, T) -> bool,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> State<T> {
    let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
    match op(i1, i2) {
        true => new_state,
        _ => State::Bottom,
    }
}
