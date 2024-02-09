use std::{
    cmp::{self, Ordering},
    fmt,
    ops::{self},
};

use crate::integer::*;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Interval {
    Bottom,
    Range(Integer, Integer),
}

impl Interval {
    pub fn shift(&self, val: Integer) -> Self {
        self.add_min(val).add_max(val)
    }

    pub fn add_min(&self, val: Integer) -> Self {
        match *self {
            Interval::Bottom => Interval::Bottom,
            Interval::Range(min, max) => Interval::Range(min + val, max),
        }
    }

    pub fn add_max(&self, val: Integer) -> Self {
        match *self {
            Interval::Bottom => Interval::Bottom,
            Interval::Range(min, max) => Interval::Range(min, max + val),
        }
    }

    pub fn union(&self, other: Self) -> Self {
        match (*self, other) {
            (a, Interval::Bottom) => a,
            (Interval::Bottom, b) => b,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(cmp::min(a, c), cmp::max(b, d))
            }
        }
    }

    pub fn intersection(&self, other: Self) -> Self {
        match (*self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                match (cmp::max(a, c), cmp::min(b, d)) {
                    (min, max) if min <= max => Interval::Range(min, max),
                    _ => Interval::Bottom,
                }
            }
        }
    }

    pub fn widen(&self, other: Self) -> Self {
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

    pub fn narrow(&self, other: Self) -> Self {
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

// --- operators

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

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Interval::Bottom, _) => Some(Ordering::Less),
            (_, Interval::Bottom) => None,
            (Interval::Range(a, b), Interval::Range(c, d)) => match a >= c && b <= d {
                true => Some(Ordering::Less),
                _ => None,
            },
        }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Interval::Bottom, Interval::Bottom) => true,
            (Interval::Bottom, _) | (_, Interval::Bottom) => false,
            (Interval::Range(a, b), Interval::Range(c, d)) => a == c && b == d,
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::Bottom => write!(f, "Empty interval"),
            // Interval::Range(a, b) if a == b => write!(f, "{}", a),
            Interval::Range(a, b) => write!(f, "[{}, {}]", a, b),
        }
    }
}
