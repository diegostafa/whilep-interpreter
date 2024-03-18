use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use crate::types::integer::*;
use crate::utils::math::*;
use std::{
    cmp, fmt,
    ops::{self, Neg},
    str::FromStr,
};

use super::expression_tree::ExpressionTree;

pub static mut LOWER_BOUND: Integer = Integer::NegInf;
pub static mut UPPER_BOUND: Integer = Integer::PosInf;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Interval {
    Empty,
    Range(Integer, Integer),
}

impl Interval {
    pub fn min(&self) -> Option<Integer> {
        match self {
            Interval::Empty => None,
            Interval::Range(a, _) => Some(*a),
        }
    }

    pub fn max(&self) -> Option<Integer> {
        match self {
            Interval::Empty => None,
            Interval::Range(_, b) => Some(*b),
        }
    }

    pub fn open_min(&self) -> Self {
        match *self {
            Interval::Empty => Interval::Empty,
            Interval::Range(_, b) => Interval::Range(Integer::NegInf, b),
        }
    }

    pub fn open_max(&self) -> Self {
        match *self {
            Interval::Empty => Interval::Empty,
            Interval::Range(a, _) => Interval::Range(a, Integer::PosInf),
        }
    }

    pub fn check_bounds(&self) -> Self {
        unsafe {
            match self.clone() {
                Interval::Empty => Interval::Empty,
                _ if self.min() == self.max() => *self,
                Interval::Range(a, b) => {
                    let min = match a {
                        _ if a < LOWER_BOUND => Integer::NegInf,
                        _ if a > UPPER_BOUND => UPPER_BOUND,
                        _ => a,
                    };
                    let max = match b {
                        _ if b > UPPER_BOUND => Integer::PosInf,
                        _ if b < LOWER_BOUND => LOWER_BOUND,
                        _ => b,
                    };

                    Interval::Range(min, max)
                }
            }
        }
    }
}

impl Domain for Interval {
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        let (val, new_state) = match expr {
            ArithmeticExpr::Number(n) => (Interval::Range(*n, *n), state.clone()),
            ArithmeticExpr::Interval(a1, a2) => {
                let (a1_val, a1_state) = Self::eval_aexpr(a1, state);
                let (a2_val, a2_state) = Self::eval_aexpr(a2, &a1_state);
                (a1_val.lub(&a2_val), a2_state)
            }
            _ => unreachable!(),
        };

        (val.check_bounds(), new_state)
    }

    fn eval_specific_bexpr(cmp_expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match cmp_expr {
            BooleanExpr::NumNotEq(a1, a2) => Self::eval_aexpr(a2, &Self::eval_aexpr(a1, state).1).1,

            BooleanExpr::NumLt(a1, a2) => {
                let a1_tree = ExpressionTree::build(a1, state).0;
                let a2_tree = ExpressionTree::build(a2, state).0;
                let (i1, i2) = (a1_tree.value(), a2_tree.value());

                let l_intersection = i1.glb(&(i2.open_min() - ONE));
                let r_intersection = i2.glb(&(i1.open_max() + ONE));

                match (l_intersection, r_intersection) {
                    (Interval::Empty, _) | (_, Interval::Empty) => State::Bottom,
                    _ => {
                        let new_state = a1_tree.refine(l_intersection, state.clone());
                        let new_state = a2_tree.refine(r_intersection, new_state);

                        let new_state = Self::eval_aexpr(a1, &new_state).1;
                        let new_state = Self::eval_aexpr(a2, &new_state).1;
                        new_state
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Lattice for Interval {
    const TOP: Self = Interval::Range(Integer::NegInf, Integer::PosInf);
    const BOT: Self = Interval::Empty;
    const UNIT: Self = Interval::Range(ONE, ONE);

    fn lub(&self, other: &Self) -> Self {
        match (*self, *other) {
            (a, Interval::Empty) => a,
            (Interval::Empty, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(cmp::min(a, c), cmp::max(b, d))
            }
        }
        .check_bounds()
    }

    fn glb(&self, other: &Self) -> Self {
        match (*self, *other) {
            (Interval::Empty, _) | (_, Interval::Empty) => Interval::Empty,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                match (cmp::max(a, c), cmp::min(b, d)) {
                    (min, max) if min <= max => Interval::Range(min, max),
                    _ => Interval::Empty,
                }
            }
        }
        .check_bounds()
    }

    fn widen(&self, other: &Self) -> Self {
        let lb = max!(Integer::NegInf, unsafe { LOWER_BOUND });
        let ub = min!(Integer::PosInf, unsafe { UPPER_BOUND });

        match (*self, *other) {
            (a, Interval::Empty) => a,
            (Interval::Empty, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = if a <= c { a } else { lb };
                let max = if b >= d { b } else { ub };
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
        .check_bounds()
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

impl FromStr for Interval {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        match s.starts_with("[") && s.ends_with("]") {
            true => {
                let s = &s[1..s.len() - 1];
                let parts: Vec<&str> = s.split(',').collect();

                match parts.len() {
                    1 => {
                        let val = Integer::from_str(parts[0]).unwrap();
                        Ok(Interval::Range(val, val))
                    }
                    2 => {
                        let min = Integer::from_str(parts[0]).unwrap();
                        let max = Integer::from_str(parts[1]).unwrap();
                        Ok(Interval::Range(min, max))
                    }
                    _ => Err("Invalid interval format, more than 2 arguments".into()),
                }
            }
            _ => Err("Invalid interval format, possibly missing brackets: [a,b]".into()),
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::Empty => write!(f, "Empty interval"),
            Interval::Range(a, b) if a == b => write!(f, "[{}]", a),
            Interval::Range(a, b) => write!(f, "[{},{}]", a, b),
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
        .check_bounds()
    }
}

impl ops::Add<Interval> for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a + c, b + d),
            _ => Interval::Empty,
        }
        .check_bounds()
    }
}

impl ops::Sub<Interval> for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a - d, b - c),
            _ => Interval::Empty,
        }
        .check_bounds()
    }
}

impl ops::Mul<Interval> for Interval {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let ac = a * c;
                let ad = a * d;
                let bd = b * d;
                let bc = b * c;
                Interval::Range(min!(ac, ad, bc, bd), max!(ac, ad, bc, bd))
            }
            _ => Interval::Empty,
        }
        .check_bounds()
    }
}

impl ops::Div<Interval> for Interval {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let one = ONE;

                match (c, d) {
                    _ if c >= one => Interval::Range(min!(a / c, a / d), max!(b / c, b / d)),
                    _ if d <= -one => Interval::Range(min!(b / c, b / d), max!(a / c, a / d)),
                    _ => {
                        let semibound = Interval::Range(one, Integer::PosInf);
                        let pos = self / other.glb(&semibound);
                        let neg = self / other.glb(&semibound.neg());
                        pos.lub(&neg)
                    }
                }
            }
            _ => Interval::Empty,
        }
        .check_bounds()
    }
}

impl ops::Add<Integer> for Interval {
    type Output = Self;

    fn add(self, other: Integer) -> Self {
        self + Interval::Range(other, other)
    }
}

impl ops::Sub<Integer> for Interval {
    type Output = Self;

    fn sub(self, other: Integer) -> Self {
        self - Interval::Range(other, other)
    }
}

impl ops::Mul<Integer> for Interval {
    type Output = Self;

    fn mul(self, other: Integer) -> Self {
        self * Interval::Range(other, other)
    }
}

impl ops::Div<Integer> for Interval {
    type Output = Self;

    fn div(self, other: Integer) -> Self {
        self / Interval::Range(other, other)
    }
}
