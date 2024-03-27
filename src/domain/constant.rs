use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use crate::types::integer::*;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Constant {
    None,
    Value(Integer),
    Any,
}

impl Lattice for Constant {
    const TOP: Self = Constant::Any;
    const BOT: Self = Constant::None;
    const UNIT: Self = Constant::Value(ONE);
    const ZERO: Self = Constant::Value(ZERO);

    fn lub(&self, other: &Self) -> Self {
        match (self, other) {
            _ if self == other => *self,
            (_, Constant::None) => *self,
            (Constant::None, _) => *other,
            _ => Constant::Any,
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self, other) {
            _ if self == other => *self,
            (_, Constant::Any) => *self,
            (Constant::Any, _) => *other,
            _ => Constant::None,
        }
    }

    fn widen(&self, other: &Self) -> Self {
        other.clone()
    }

    fn narrow(&self, other: &Self) -> Self {
        other.clone()
    }

    fn round(x: &Self) -> Self {
        match *x {
            Constant::None => Constant::None,
            _ => Constant::Any,
        }
    }
}

impl Domain for Constant {
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        match expr {
            ArithmeticExpr::Number(c) => (Constant::Value(*c), state.clone()),
            ArithmeticExpr::Interval(a1, a2) => {
                let (a1_val, new_state) = Self::eval_aexpr(a1, state);
                let (a2_val, new_state) = Self::eval_aexpr(a2, &new_state);
                match a1_val {
                    _ if a1_val == a2_val => (a1_val, new_state),
                    _ if a1_val > a2_val => (Constant::None, new_state),
                    _ => (Constant::Any, new_state),
                }
            }
            _ => unreachable!(),
        }
    }

    fn eval_specific_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match expr {
            BooleanExpr::NumNotEq(a1, a2) => {
                let (lhs, new_state) = Self::eval_aexpr(a1, &state);
                let (rhs, new_state) = Self::eval_aexpr(a2, &new_state);
                match (lhs, rhs) {
                    (Constant::None, _) | (_, Constant::None) => State::Bottom,
                    (Constant::Any, _) | (_, Constant::Any) => new_state,
                    (Constant::Value(l), Constant::Value(r)) => match l != r {
                        true => new_state,
                        _ => State::Bottom,
                    },
                }
            }
            BooleanExpr::NumLt(a1, a2) => {
                let (lhs, new_state) = Self::eval_aexpr(a1, &state);
                let (rhs, new_state) = Self::eval_aexpr(a2, &new_state);
                match (lhs, rhs) {
                    (Constant::None, _) | (_, Constant::None) => State::Bottom,
                    (Constant::Any, _) | (_, Constant::Any) => new_state,
                    (Constant::Value(l), Constant::Value(r)) => match l < r {
                        true => new_state,
                        _ => State::Bottom,
                    },
                }
            }

            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::None => write!(f, "Bottom constant"),
            Constant::Value(c) => write!(f, "{}", c),
            Constant::Any => write!(f, "Any"),
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
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a * b),
            (Constant::Any, Constant::Value(a)) | (Constant::Value(a), Constant::Any)
                if a == ZERO =>
            {
                Constant::Value(ZERO)
            }
            _ => Constant::Any,
        }
    }
}

impl ops::Div<Constant> for Constant {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (_, Constant::Value(b)) if b == ZERO => Constant::None,
            (Constant::Value(a), _) if a == ZERO => Constant::Value(ZERO),
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a / b),
            _ => Constant::Any,
        }
    }
}
