use std::{
    fmt::{self},
    ops::{self},
    str::FromStr,
};

use rand::Rng;

pub enum Sign {
    Neg,
    Zero,
    Pos,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Integer {
    NegInf,
    Value(i32),
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
            _ => match s.parse::<i32>() {
                Ok(v) => Ok(Integer::Value(v)),
                Err(e) => Err(e),
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

            _ => panic!("[ERROR] PosInf + NegInf"),
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
        let mul_sign = |i: Integer, sign: Sign| match sign {
            Sign::Pos => i,
            Sign::Zero => Integer::Value(0),
            Sign::Neg => -i,
        };

        match (self, other) {
            (Integer::Value(a), Integer::Value(b)) => Integer::Value(a * b),
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

impl ops::Add<i32> for Integer {
    type Output = Self;

    fn add(self, int: i32) -> Self {
        match self {
            Integer::PosInf => Integer::PosInf,
            Integer::NegInf => Integer::NegInf,
            Integer::Value(n) => match n.checked_add(int) {
                Some(val) => Integer::Value(val),
                None => Integer::PosInf,
            },
        }
    }
}

impl ops::Sub<i32> for Integer {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        self + -other
    }
}

fn sign(i: Integer) -> Sign {
    match i {
        Integer::NegInf => Sign::Neg,
        Integer::Value(n) if n < 0 => Sign::Neg,
        Integer::Value(n) if n == 0 => Sign::Zero,
        Integer::Value(n) if n > 0 => Sign::Pos,
        Integer::PosInf => Sign::Pos,
        _ => unreachable!(),
    }
}

pub fn random_integer_between(min: Integer, max: Integer) -> Integer {
    let rng: f32 = rand::thread_rng().gen();
    let min = match min {
        Integer::Value(v) => v,
        Integer::NegInf => std::i32::MIN / 2,
        Integer::PosInf => std::i32::MAX / 2,
    };

    let max = match max {
        Integer::Value(v) => v,
        Integer::NegInf => std::i32::MIN / 2,
        Integer::PosInf => std::i32::MAX / 2,
    };

    Integer::Value((rng * (max - min) as f32) as i32 + min)
}
