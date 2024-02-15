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

pub static mut LOWER_BOUND: Integer = Integer::NegInf;
pub static mut UPPER_BOUND: Integer = Integer::PosInf;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Interval {
    Empty,
    Range(Integer, Integer),
}

impl Interval {
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

    pub fn check_bounds(&self) -> Self {
        self.check_bound_left().check_bound_right()
    }

    fn check_bound_left(&self) -> Self {
        unsafe {
            match *self {
                Interval::Empty => Interval::Empty,
                _ if LOWER_BOUND == Integer::NegInf => *self,
                Interval::Range(a, b) => {
                    let min = if a < LOWER_BOUND { LOWER_BOUND } else { a };
                    Interval::Range(min, b)
                }
            }
        }
    }

    fn check_bound_right(&self) -> Self {
        unsafe {
            match *self {
                Interval::Empty => Interval::Empty,
                _ if UPPER_BOUND == Integer::PosInf => *self,
                Interval::Range(a, b) => {
                    let max = if b > UPPER_BOUND { UPPER_BOUND } else { b };
                    Interval::Range(a, max)
                }
            }
        }
    }
}

impl Domain for Interval {
    fn eval_specific_aexpr(expr: &ArithmeticExpr, state: &State<Self>) -> (Self, State<Self>) {
        let (val, new_state) = match expr {
            ArithmeticExpr::Number(n) => (Interval::Range(*n, *n), state.clone()),
            ArithmeticExpr::Interval(n, m) => match n <= m {
                true => (Interval::Range(*n, *m), state.clone()),
                _ => (Interval::Empty, state.clone()),
            },
            _ => unreachable!(),
        };

        (val.check_bounds(), new_state)
    }

    fn eval_specific_bexpr(cmp_expr: &BooleanExpr, state: &State<Self>) -> State<Self> {
        match cmp_expr {
            BooleanExpr::NumEq(a1, a2) => {
                let (ltree, _) = Self::build_expression_tree(a1, state);
                let (rtree, _) = Self::build_expression_tree(a2, state);
                let (i1, i2) = (ltree.get_value(), rtree.get_value());

                match i1.intersection(&i2) {
                    Interval::Empty => State::Bottom,
                    intersection => {
                        let new_state = state
                            .refine_expression_tree(&ltree, intersection)
                            .refine_expression_tree(&rtree, intersection);

                        let a1_state = Self::eval_aexpr(a1, &new_state).1;
                        let a2_state = Self::eval_aexpr(a2, &a1_state).1;
                        a2_state
                    }
                }
            }
            BooleanExpr::NumNotEq(a1, a2) => {
                let (_, new_state) = Self::eval_aexpr(a1, state);
                let (_, new_state) = Self::eval_aexpr(a2, &new_state);
                new_state
            }
            BooleanExpr::NumLt(a1, a2) => {
                let (ltree, _) = Self::build_expression_tree(a1, state);
                let (rtree, _) = Self::build_expression_tree(a2, state);
                let (i1, i2) = (ltree.get_value(), rtree.get_value());

                let one = Integer::Value(1);
                let lhs = i1.intersection(&i2.add_min(Integer::NegInf).add_max(-one));
                let rhs = i2.intersection(&i1.add_max(Integer::PosInf).add_min(one));

                match (lhs, rhs) {
                    (Interval::Empty, _) | (_, Interval::Empty) => State::Bottom,
                    _ => {
                        let new_state = state
                            .refine_expression_tree(&ltree, lhs)
                            .refine_expression_tree(&rtree, rhs);

                        let a1_state = Self::eval_aexpr(a1, &new_state).1;
                        let a2_state = Self::eval_aexpr(a2, &a1_state).1;
                        a2_state
                    }
                }
            }
            _ => unreachable!("{}", cmp_expr),
        }
    }
}

impl Lattice for Interval {
    const TOP: Self = Interval::Range(Integer::NegInf, Integer::PosInf);
    const BOT: Self = Interval::Empty;
    const UNIT: Self = Interval::Range(Integer::Value(1), Integer::Value(1));

    fn union(&self, other: &Self) -> Self {
        match (*self, *other) {
            (a, Interval::Empty) => a,
            (Interval::Empty, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(cmp::min(a, c), cmp::max(b, d))
            }
        }
        .check_bounds()
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
        .check_bounds()
    }

    fn widen(&self, other: &Self) -> Self {
        unsafe {
            if LOWER_BOUND != Integer::NegInf || UPPER_BOUND != Integer::PosInf {
                return self.union(other);
            }
        }

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
                let ac = a * c;
                let ad = a * d;
                let bd = b * d;
                let bc = b * c;
                Interval::Range(min!(ac, ad, bc, bd), max!(ac, ad, bc, bd))
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

                match (c, d) {
                    _ if c >= one => Interval::Range(min!(a / c, a / d), max!(b / c, b / d)),
                    _ if d <= -one => Interval::Range(min!(b / c, b / d), max!(a / c, a / d)),
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
