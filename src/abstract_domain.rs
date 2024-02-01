use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub enum Integer {
    PlusInfinity,
    Value(i32),
    MinusInfinity,
}
impl Add for Integer {
    type Output = Integer;

    fn add(self, other: Integer) -> Integer {
        match (self, other) {
            (Integer::Value(a), Integer::Value(b)) => Integer::Value(a + b),
            (inf, Integer::Value(_)) => inf,
            (Integer::Value(_), inf) => inf,
            _ => panic!("[ERROR] PlusInfinity + MinusInfinity"),
        }
    }
}

impl Integer {
    pub fn from(val: i32) -> Integer {
        Integer::Value(val)
    }

    pub fn add(self, val: i32) -> Integer {
        match self {
            Integer::PlusInfinity => Integer::PlusInfinity,
            Integer::Value(v) => Integer::Value(v + val),
            Integer::MinusInfinity => Integer::MinusInfinity,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Top,
    Range(Integer, Integer),
    Bottom,
}

impl Interval {
    pub fn from_value(val: i32) -> Interval {
        Interval::Range(Integer::from(val), Integer::from(val))
    }

    pub fn from_bounds(min: Option<i32>, max: Option<i32>) -> Interval {
        match (min, max) {
            (Some(min), Some(max)) => Interval::Range(Integer::from(min), Integer::from(max)),
            (Some(min), _) => Interval::Range(Integer::from(min), Integer::PlusInfinity),
            (_, Some(max)) => Interval::Range(Integer::MinusInfinity, Integer::from(max)),
            (_, _) => Interval::Range(Integer::MinusInfinity, Integer::PlusInfinity),
        }
    }

    pub fn add_value(&self, val: i32) -> Interval {
        match self {
            Interval::Top => Interval::Top,
            Interval::Range(min, max) => Interval::Range(min.add(val), *max),
            Interval::Bottom => Interval::Bottom,
        }
    }
}
