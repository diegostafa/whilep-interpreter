use crate::types::sign::*;
use rand::Rng;
use std::{
    fmt::{self},
    ops::{self},
    str::FromStr,
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Integer {
    NegInf,
    Value(i64),
    PosInf,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Integer::NegInf => write!(f, "neginf"),
            Integer::Value(v) => write!(f, "{}", v),
            Integer::PosInf => write!(f, "posinf"),
        }
    }
}

impl FromStr for Integer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "neginf" => Ok(Integer::NegInf),
            "posinf" => Ok(Integer::PosInf),
            _ => match s.parse::<i64>() {
                Ok(v) => Ok(Integer::Value(v)),
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => Ok(Integer::PosInf),
                    std::num::IntErrorKind::NegOverflow => Ok(Integer::NegInf),
                    _ => Err(e),
                },
            },
        }
    }
}

impl ops::Neg for Integer {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Integer::NegInf => Integer::PosInf,
            Integer::PosInf => Integer::NegInf,
            Integer::Value(v) => Integer::Value(-v),
        }
    }
}

impl ops::Add<Integer> for Integer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Integer::Value(_), Integer::Value(b)) => self + b,

            (Integer::PosInf, Integer::PosInf)
            | (Integer::PosInf, Integer::Value(_))
            | (Integer::Value(_), Integer::PosInf) => Integer::PosInf,

            (Integer::NegInf, Integer::NegInf)
            | (Integer::NegInf, Integer::Value(_))
            | (Integer::Value(_), Integer::NegInf) => Integer::NegInf,

            _ => panic!("[ERROR] undefined operation: PosInf + NegInf"),
        }
    }
}

impl ops::Sub<Integer> for Integer {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl ops::Mul<Integer> for Integer {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Integer::Value(_), Integer::Value(b)) => self * b,
            (Integer::Value(_), _) => mul_sign(other, sign(self)),
            (_, Integer::Value(_)) => mul_sign(self, sign(other)),
            (inf1, inf2) => mul_sign(inf1, sign(inf2)),
        }
    }
}

impl ops::Div<Integer> for Integer {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let zero = Integer::Value(0);
        match (self, other) {
            (a, _) if a == zero => zero,
            (_, Integer::PosInf) | (_, Integer::NegInf) => zero,
            (Integer::NegInf, b) if b > zero => Integer::NegInf,
            (Integer::NegInf, b) if b < zero => Integer::PosInf,
            (Integer::PosInf, b) if b < zero => Integer::NegInf,
            (Integer::PosInf, b) if b > zero => Integer::NegInf,
            (Integer::Value(a), b) if a > 0 && b == zero => Integer::PosInf,
            (Integer::Value(a), b) if a < 0 && b == zero => Integer::NegInf,
            (Integer::Value(a), Integer::Value(b)) => Integer::Value(a / b),
            _ => unreachable!(),
        }
    }
}

impl ops::Add<i64> for Integer {
    type Output = Self;

    fn add(self, i: i64) -> Self {
        match self {
            Integer::PosInf => Integer::PosInf,
            Integer::NegInf => Integer::NegInf,
            Integer::Value(n) => match n.checked_add(i) {
                Some(val) => Integer::Value(val),
                None => Integer::PosInf,
            },
        }
    }
}

impl ops::Sub<i64> for Integer {
    type Output = Self;

    fn sub(self, other: i64) -> Self {
        self + -other
    }
}

impl ops::Mul<i64> for Integer {
    type Output = Self;

    fn mul(self, i: i64) -> Self {
        let new_sign = mul_sign(self, sign_i64(i));
        match self {
            Integer::PosInf | Integer::NegInf => new_sign,
            Integer::Value(n) => match n.checked_mul(i) {
                Some(val) => Integer::Value(val),
                None => match sign(self) {
                    Sign::Neg if i > 0 => Integer::NegInf,
                    Sign::Neg if i < 0 => Integer::PosInf,
                    Sign::Pos if i > 0 => Integer::PosInf,
                    Sign::Pos if i < 0 => Integer::NegInf,
                    _ => unreachable!(),
                },
            },
        }
    }
}

pub fn random_integer_between(min: Integer, max: Integer) -> Integer {
    if min > max {
        panic!("[ERROR] invalid interval: [{}, {}]", min, max);
    }

    let rng: f32 = rand::thread_rng().gen();
    let min = match min {
        Integer::Value(v) => v,
        Integer::NegInf => std::i64::MIN / 2,
        Integer::PosInf => std::i64::MAX / 2,
    };

    let max = match max {
        Integer::Value(v) => v,
        Integer::NegInf => std::i64::MIN / 2,
        Integer::PosInf => std::i64::MAX / 2,
    };

    Integer::Value((rng * (max - min) as f32) as i64 + min)
}
