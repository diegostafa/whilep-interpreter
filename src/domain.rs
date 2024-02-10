use std::fmt::Display;

use crate::abstract_state::*;
use crate::ast::*;
use crate::lattice::Lattice;

pub trait Domain: Sized + Lattice + Display + Clone + Eq {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>);
    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self>;
}

pub fn trans_aexpr<T: Domain + Lattice + Display + Clone>(
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> (T, T, State<T>) {
    let (i1, new_state) = T::eval_aexpr(a1, &state);
    let (i2, new_state) = T::eval_aexpr(a2, &new_state);
    (i1, i2, new_state)
}

pub fn binop_aexpr<T: Domain + Lattice + Display + Clone>(
    op: fn(T, T) -> T,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State<T>,
) -> (T, State<T>) {
    let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
    (op(i1, i2), new_state)
}
