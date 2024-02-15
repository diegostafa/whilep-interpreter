#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {{
        let temp_min = $x;
        let next_min = min!($($y),+);
        std::cmp::min(temp_min, next_min)
    }};
}

#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {{
        let temp_max = $x;
        let next_max = max!($($y),+);
        std::cmp::max(temp_max, next_max)
    }};
}

pub(crate) use max;
pub(crate) use min;
