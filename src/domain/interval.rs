use std::{
    cmp, fmt,
    ops::{self, Neg},
};

use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use crate::types::integer::*;

pub static mut LOWER_BOUND: Integer = Integer::NegInf;
pub static mut UPPER_BOUND: Integer = Integer::PosInf;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Interval {
    Empty,
    Range(Integer, Integer),
}

impl Interval {
    pub fn shift(&self, val: Integer) -> Self {
        self.add_min(val).add_max(val)
    }

    pub fn add_min(&self, val: Integer) -> Self {
        match *self {
            Interval::Empty => Interval::Empty,
            Interval::Range(a, b) => Interval::Range(a + val, b),
        }
    }

    pub fn add_max(&self, val: Integer) -> Self {
        match *self {
            Interval::Empty => Interval::Empty,
            Interval::Range(a, b) => Interval::Range(a, b + val),
        }
    }

    pub fn clamp(&self) -> Self {
        match *self {
            Interval::Empty => Interval::Empty,
            Interval::Range(a, b) => unsafe {
                let min = if a < LOWER_BOUND { Integer::NegInf } else { a };
                let max = if b > UPPER_BOUND { Integer::PosInf } else { b };
                Interval::Range(min, max)
            },
        }
    }
}

impl Domain for Interval {
    fn eval_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        match expr {
            ArithmeticExpr::Number(n) => (Interval::Range(*n, *n), state.clone()),
            ArithmeticExpr::Interval(n, m) => (Interval::Range(*n, *m), state.clone()),
            ArithmeticExpr::Identifier(var) => (state.read(var), state.clone()),
            ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
            ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
            ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
            ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),
            ArithmeticExpr::PostIncrement(var) => {
                let interval = state.read(var);
                (interval, state.put(var, interval.shift(Integer::Value(1))))
            }
            ArithmeticExpr::PostDecrement(var) => {
                let interval = state.read(var);
                (interval, state.put(var, interval.shift(Integer::Value(-1))))
            }
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
            BooleanExpr::NumEq(a1, a2) => {
                let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
                match i1.intersection(&i2) {
                    Interval::Empty => State::Bottom,
                    intersection => new_state
                        .try_put(a1, intersection)
                        .try_put(a2, intersection),
                }
            }
            BooleanExpr::NumNotEq(a1, a2) => {
                let (_, _, new_state) = trans_aexpr(a1, a2, &state);
                new_state
            }
            BooleanExpr::NumLt(a1, a2) => {
                let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
                let one = Integer::Value(1);
                let lhs = i1.intersection(&i2.add_min(Integer::NegInf).add_max(-one));
                let rhs = i2.intersection(&i1.add_max(Integer::PosInf).add_min(one));
                match (lhs, rhs) {
                    (Interval::Empty, _) | (_, Interval::Empty) => State::Bottom,
                    _ => new_state.try_put(a1, lhs).try_put(a2, rhs),
                }
            }
            BooleanExpr::NumLtEq(a1, a2) => {
                let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
                let lhs = i1.intersection(&i2.add_min(Integer::NegInf));
                let rhs = i2.intersection(&i1.add_max(Integer::PosInf));
                match (lhs, rhs) {
                    (Interval::Empty, _) | (_, Interval::Empty) => State::Bottom,
                    _ => new_state.try_put(a1, lhs).try_put(a2, rhs),
                }
            }
            BooleanExpr::NumGt(a1, a2) => {
                Self::eval_bexpr(&BooleanExpr::NumLt(a2.clone(), a1.clone()), state)
            }
            BooleanExpr::NumGtEq(a1, a2) => {
                Self::eval_bexpr(&BooleanExpr::NumLtEq(a2.clone(), a1.clone()), state)
            }
        }
    }
}

impl Lattice for Interval {
    const TOP: Self = Interval::Range(Integer::NegInf, Integer::PosInf);
    const BOT: Self = Interval::Empty;

    fn union(&self, other: &Self) -> Self {
        match (*self, *other) {
            (a, Interval::Empty) => a,
            (Interval::Empty, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(cmp::min(a, c), cmp::max(b, d))
            }
        }
        .clamp()
    }

    fn intersection(&self, other: &Self) -> Self {
        match (*self, *other) {
            (Interval::Empty, _) | (_, Interval::Empty) => Interval::Empty,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                match (cmp::max(a, c), cmp::min(b, d)) {
                    (min, max) if min <= max => Interval::Range(min, max),
                    _ => Interval::Empty,
                }
            }
        }
        .clamp()
    }

    fn widen(&self, other: &Self) -> Self {
        match (*self, *other) {
            (a, Interval::Empty) => a,
            (Interval::Empty, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = if a <= c { a } else { Integer::NegInf };
                let max = if b >= d { b } else { Integer::PosInf };
                Interval::Range(min, max)
            }
        }
    }

    fn narrow(&self, other: &Self) -> Self {
        match (*self, *other) {
            (Interval::Empty, _) | (_, Interval::Empty) => Interval::Empty,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = if a == Integer::NegInf { c } else { a };
                let max = if b == Integer::PosInf { d } else { b };
                Interval::Range(min, max)
            }
        }
        .clamp()
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Interval::Empty, Interval::Empty) => true,
            (Interval::Empty, _) | (_, Interval::Empty) => false,
            (Interval::Range(a, b), Interval::Range(c, d)) => a == c && b == d,
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::Empty => write!(f, "Empty interval"),
            Interval::Range(a, b) if a == b => write!(f, "[{}]", a),
            Interval::Range(a, b) => write!(f, "[{}, {}]", a, b),
        }
    }
}

impl ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Interval::Empty => self,
            Interval::Range(a, b) => Interval::Range(-b, -a),
        }
    }
}

impl ops::Add<Interval> for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a + c, b + d),
            _ => Interval::Empty,
        }
    }
}

impl ops::Sub<Interval> for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a - d, b - c),
            _ => Interval::Empty,
        }
    }
}

impl ops::Mul<Interval> for Interval {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let bounds = [a * c, a * d, b * c, b * d];
                let min = bounds.iter().min().unwrap();
                let max = bounds.iter().max().unwrap();
                Interval::Range(*min, *max)
            }
            _ => Interval::Empty,
        }
    }
}

impl ops::Div<Interval> for Interval {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let one = Integer::Value(1);
                let abounds = [a * c, a * d];
                let bbounds = [b * c, b * d];

                match (c, d) {
                    _ if c >= one => {
                        let min = abounds.iter().min().unwrap();
                        let max = bbounds.iter().max().unwrap();
                        Interval::Range(*min, *max)
                    }
                    _ if d <= -one => {
                        let min = bbounds.iter().min().unwrap();
                        let max = abounds.iter().max().unwrap();
                        Interval::Range(*min, *max)
                    }
                    _ => {
                        let semibound = Interval::Range(one, Integer::PosInf);
                        let pos = self / other.intersection(&semibound);
                        let neg = self / other.intersection(&semibound.neg());
                        pos.union(&neg)
                    }
                }
            }
            _ => Interval::Empty,
        }
    }
}
