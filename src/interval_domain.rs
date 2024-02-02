use crate::integer::*;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Range(Integer, Integer),
    Bottom,
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

pub fn interval_union(i1: Interval, i2: Interval) -> Interval {
    match (i1, i2) {
        (Interval::Bottom, _) => i2,
        (_, Interval::Bottom) => i1,
        (Interval::Range(a, b), Interval::Range(c, d)) => {
            Interval::Range(min_integer(a, c), max_integer(b, d))
        }
    }
}

pub fn interval_intersection(i1: Interval, i2: Interval) -> Interval {
    match (i1, i2) {
        (Interval::Bottom, _) => i1,
        (_, Interval::Bottom) => i2,
        (Interval::Range(a, b), Interval::Range(c, d)) => {
            let max = max_integer(a, c);
            let min = min_integer(b, d);

            Interval::Range(min_integer(a, c), max_integer(b, d))
        }
    }
}

pub fn interval_add_value(i: Interval, val: i32) -> Interval {
    match i {
        Interval::Range(min, max) => Interval::Range(min + val, max),
        Interval::Bottom => Interval::Bottom,
    }
}

pub fn interval_contains(i1: Interval, i2: Interval) -> bool {
    match (i1, i2) {
        (Interval::Bottom, _) | (_, Interval::Bottom) => true,
        (Interval::Range(a, b), Interval::Range(c, d)) => a <= c && b >= d,
    }
}
