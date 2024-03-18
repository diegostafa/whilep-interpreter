use crate::types::integer::*;

pub enum Sign {
    Neg,
    Zero,
    Pos,
}

pub fn sign(i: Integer) -> Sign {
    match i {
        Integer::NegInf => Sign::Neg,
        Integer::Value(n) if n < 0 => Sign::Neg,
        Integer::Value(n) if n == 0 => Sign::Zero,
        Integer::Value(n) if n > 0 => Sign::Pos,
        Integer::PosInf => Sign::Pos,
        _ => unreachable!(),
    }
}

pub fn sign_i64(i: i64) -> Sign {
    match i {
        _ if i < 0 => Sign::Neg,
        _ if i == 0 => Sign::Zero,
        _ if i > 0 => Sign::Pos,
        _ => unreachable!(),
    }
}
pub fn mul_sign(i: Integer, sign: Sign) -> Integer {
    match sign {
        Sign::Neg => -i,
        Sign::Zero => ZERO,
        Sign::Pos => i,
    }
}
