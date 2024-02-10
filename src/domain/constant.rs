use std::fmt;

use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use crate::types::integer::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    Bottom,
    Value(Integer),
    Top,
}

impl Lattice for Constant {
    const TOP: Self = Constant::Top;
    const BOT: Self = Constant::Bottom;

    fn union(&self, other: &Self) -> Self {
        *self
    }

    fn intersection(&self, other: &Self) -> Self {
        *self
    }

    fn widen(&self, other: &Self) -> Self {
        *self
    }

    fn narrow(&self, other: &Self) -> Self {
        *self
    }
}

impl Domain for Constant {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        todo!()
    }

    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        todo!()
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Bottom => write!(f, "Bottom constant"),
            Constant::Value(c) => write!(f, "{}", c),
            Constant::Top => write!(f, "Top constant"),
        }
    }
}
