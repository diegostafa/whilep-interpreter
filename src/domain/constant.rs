use std::fmt;
use std::ops;

use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use crate::types::integer::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    None,
    Value(Integer),
    Any,
}

impl Lattice for Constant {
    const TOP: Self = Constant::Any;
    const BOT: Self = Constant::None;

    fn union(&self, other: &Self) -> Self {
        match self == other {
            true => *self,
            _ => Constant::Any,
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        match self == other {
            true => *self,
            _ => Constant::None,
        }
    }

    fn widen(&self, other: &Self) -> Self {
        self.union(other)
    }

    fn narrow(&self, other: &Self) -> Self {
        self.intersection(other)
    }
}

impl Domain for Constant {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        match expr {
            ArithmeticExpr::Number(c) => (Constant::Value(*c), state.clone()),
            ArithmeticExpr::Interval(_, _) => (Constant::Any, state.clone()),
            ArithmeticExpr::Identifier(_) => todo!(),
            ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
            ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
            ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
            ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),
            ArithmeticExpr::PostIncrement(_) => todo!(),
            ArithmeticExpr::PostDecrement(_) => todo!(),
        }
    }

    fn eval_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match expr {
            BooleanExpr::True => state.clone(),
            BooleanExpr::False => State::Bottom,
            BooleanExpr::Not(b) => Self::eval_bexpr(&desugar_not_bexpr(*b.clone()), state),
            BooleanExpr::And(b1, b2) => {
                Self::eval_bexpr(b1, state).intersection(&Self::eval_bexpr(b2, state))
            }
            BooleanExpr::Or(b1, b2) => {
                Self::eval_bexpr(b1, state).union(&Self::eval_bexpr(b2, state))
            }
            BooleanExpr::NumEq(_, _) => todo!(),
            BooleanExpr::NumNotEq(_, _) => todo!(),
            BooleanExpr::NumLt(_, _) => todo!(),
            BooleanExpr::NumGt(_, _) => todo!(),
            BooleanExpr::NumLtEq(_, _) => todo!(),
            BooleanExpr::NumGtEq(_, _) => todo!(),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::None => write!(f, "Bottom constant"),
            Constant::Value(c) => write!(f, "{}", c),
            Constant::Any => write!(f, "Any constant"),
        }
    }
}

impl ops::Neg for Constant {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Constant::Value(a) => Constant::Value(-a),
            _ => self,
        }
    }
}

impl ops::Add<Constant> for Constant {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (Constant::Any, _) | (_, Constant::Any) => Constant::Any,
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a + b),
        }
    }
}

impl ops::Sub<Constant> for Constant {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl ops::Mul<Constant> for Constant {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let zero = Integer::Value(0);
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (Constant::Value(a), Constant::Value(b)) if a == zero || b == zero => {
                Constant::Value(zero)
            }
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a * b),
            (Constant::Any, _) | (_, Constant::Any) => Constant::Any,
        }
    }
}

impl ops::Div<Constant> for Constant {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (Constant::Any, _) | (_, Constant::Any) => Constant::Any,
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a + b),
        }
    }
}
