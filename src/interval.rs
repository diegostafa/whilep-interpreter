use std::{
    cmp::{self, Ordering},
    ops,
};

use crate::{integer::*, lattice::Lattice};

#[derive(Debug, Clone, Copy, Eq)]
pub enum Interval {
    Bottom,
    Range(Integer, Integer),
}

pub trait IntervalOperations {
    fn add_i32(&self, other: i32) -> Self;
    fn contains(&self, other: Self) -> bool;
    fn on_left(&self, other: Self) -> bool;
    fn on_right(&self, other: Self) -> bool;
    fn overlaps(&self, other: Self) -> bool;
}

impl ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Interval::Bottom => self,
            Interval::Range(a, b) => Interval::Range(-b, -a),
        }
    }
}

impl ops::Add<Interval> for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a + c, b + d),
            _ => Interval::Bottom,
        }
    }
}

impl ops::Sub<Interval> for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => Interval::Range(a - d, b - c),
            _ => Interval::Bottom,
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
            _ => Interval::Bottom,
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
                        let pos = self / other.intersection(semibound);
                        let neg = self / other.intersection(-semibound);
                        pos.union(neg)
                    }
                }
            }
            _ => Interval::Bottom,
        }
    }
}

// inclusion ordering
impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => match a >= c && b <= d {
                true => Some(Ordering::Less),
                _ => None,
            },
            (Interval::Bottom, _) | (_, Interval::Bottom) => None,
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Interval::Range(a, b), Interval::Range(c, d)) => a == b && c == d,
            (Interval::Bottom, Interval::Bottom) => true,
            _ => false,
        }
    }
}

impl IntervalOperations for Interval {
    fn add_i32(&self, val: i32) -> Self {
        match *self {
            Interval::Bottom => Interval::Bottom,
            Interval::Range(min, max) => Interval::Range(min + val, max),
        }
    }
    fn contains(&self, other: Self) -> bool {
        match (*self, other) {
            (Interval::Bottom, _) => false,
            (_, Interval::Bottom) => true,
            (Interval::Range(a, b), Interval::Range(c, d)) => a <= c && b >= d,
        }
    }

    fn on_left(&self, other: Self) -> bool {
        match (*self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => false,
            (Interval::Range(_, b), Interval::Range(c, _)) => b <= c,
        }
    }

    fn on_right(&self, other: Self) -> bool {
        match (*self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => false,
            (Interval::Range(a, _), Interval::Range(_, d)) => a >= d,
        }
    }

    fn overlaps(&self, other: Self) -> bool {
        match self.intersection(other) {
            Interval::Bottom => false,
            _ => true,
        }
    }
}

impl Lattice for Interval {
    fn union(&self, other: Self) -> Self {
        match (*self, other) {
            (a, Interval::Bottom) => a,
            (Interval::Bottom, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(cmp::min(a, c), cmp::max(b, d))
            }
        }
    }

    fn intersection(&self, other: Self) -> Self {
        match (*self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = cmp::max(a, c);
                let max = cmp::min(b, d);
                match min < max {
                    true => Interval::Range(min, max),
                    _ => Interval::Bottom,
                }
            }
        }
    }

    fn widen(&self, other: Self) -> Self {
        match (*self, other) {
            (a, Interval::Bottom) => a,
            (Interval::Bottom, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = if a <= c { a } else { Integer::NegInf };
                let max = if b >= d { b } else { Integer::PosInf };
                Interval::Range(min, max)
            }
        }
    }

    fn narrow(&self, other: Self) -> Self {
        match (*self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let min = if a == Integer::NegInf { c } else { a };
                let max = if b == Integer::PosInf { d } else { b };
                Interval::Range(min, max)
            }
        }
    }
}

pub fn interval_from_value(val: i32) -> Interval {
    Interval::Range(Integer::Value(val), Integer::Value(val))
}

pub fn interval_from_bounds(min: Option<i32>, max: Option<i32>) -> Interval {
    match (min, max) {
        (Some(min), Some(max)) => Interval::Range(Integer::Value(min), Integer::Value(max)),
        (Some(min), _) => Interval::Range(Integer::Value(min), Integer::PosInf),
        (_, Some(max)) => Interval::Range(Integer::NegInf, Integer::Value(max)),
        (_, _) => Interval::Range(Integer::NegInf, Integer::PosInf),
    }
}
