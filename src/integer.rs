use std::{cmp, ops};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Integer {
    PosInf,
    Value(i32),
    NegInf,
}

impl ops::Add<Integer> for Integer {
    type Output = Self;

    fn add(self, other: Integer) -> Integer {
        match (self, other) {
            (Integer::Value(a), Integer::Value(b)) => Integer::Value(a + b),
            (inf, Integer::Value(_)) | (Integer::Value(_), inf) => inf,
            _ => panic!("[ERROR] PosInf + NegInf"),
        }
    }
}

impl ops::Add<i32> for Integer {
    type Output = Self;

    fn add(self, other: i32) -> Integer {
        match (self) {
            Integer::PosInf => Integer::PosInf,
            Integer::Value(v) => Integer::Value(v + other),
            Integer::NegInf => Integer::NegInf,
        }
    }
}

pub fn min_integer(a: Integer, b: Integer) -> Integer {
    match (a, b) {
        (Integer::PosInf, _) | (_, Integer::NegInf) => a,
        (_, Integer::PosInf) | (Integer::NegInf, _) => b,
        (Integer::Value(a), Integer::Value(b)) => Integer::Value(cmp::min(a, b)),
    }
}

pub fn max_integer(a: Integer, b: Integer) -> Integer {
    match (a, b) {
        (Integer::PosInf, _) | (_, Integer::NegInf) => a,
        (_, Integer::PosInf) | (Integer::NegInf, _) => b,
        (Integer::Value(a), Integer::Value(b)) => Integer::Value(cmp::max(a, b)),
    }
}
