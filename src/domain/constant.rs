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
    const UNIT: Self = Constant::Value(Integer::Value(1));

    fn union(&self, other: &Self) -> Self {
        match (self, other) {
            _ if self == other => *self,
            (_, Constant::None) => *self,
            (Constant::None, _) => *other,
            _ => Constant::Any,
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        match (self, other) {
            _ if self == other => *self,
            (_, Constant::Any) => *self,
            (Constant::Any, _) => *other,
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
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        match expr {
            ArithmeticExpr::Number(c) => (Constant::Value(*c), state.clone()),
            ArithmeticExpr::Interval(_, _) => (Constant::Any, state.clone()),
            _ => unreachable!(),
        }
    }

    fn eval_specific_bexpr(expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match expr {
            BooleanExpr::NumEq(a1, a2) => {
                let (ltree, new_state) = Self::build_expression_tree(a1, state);
                let (rtree, new_state) = Self::build_expression_tree(a2, &new_state);
                let (i1, i2) = (ltree.get_value(), rtree.get_value());

                match i1.intersection(&i2) {
                    Constant::None => State::Bottom,
                    intersection => new_state
                        .refine_expression_tree(&ltree, intersection)
                        .refine_expression_tree(&rtree, intersection),
                }
            }
            BooleanExpr::NumNotEq(a1, a2) => binop_cmp(|a, b| a != b, a1, a2, state),
            BooleanExpr::NumLt(a1, a2) => binop_cmp(|a, b| a < b, a1, a2, state),

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
        let zero = Integer::Value(0);
        match (self, other) {
            (Constant::None, _) | (_, Constant::None) => Constant::None,
            (Constant::Any, _) | (_, Constant::Any) => Constant::Any,
            (Constant::Value(_), Constant::Value(b)) if b == zero => Constant::None,
            (Constant::Value(a), Constant::Value(_)) if a == zero => Constant::Value(zero),
            (Constant::Value(a), Constant::Value(b)) => Constant::Value(a / b),
        }
    }
}
