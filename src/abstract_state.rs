use crate::integer::Integer;
use crate::interval::*;
use crate::lattice::Lattice;
use std::collections::HashMap;

pub type State = Option<HashMap<String, Interval>>;
pub type Invariant = Vec<State>;

pub trait IO {
    fn read(&self, var: &String) -> Interval;
    fn put(&self, var: &String, val: Interval) -> Self;
}

impl IO for State {
    fn read(&self, var: &String) -> Interval {
        match self {
            Some(state) => match state.get(var) {
                Some(val) => *val,
                None => Interval::Range(Integer::NegInf, Integer::PosInf),
            },
            None => panic!("[ERROR] bottom state"),
        }
    }

    fn put(&self, var: &String, val: Interval) -> Self {
        match self.clone() {
            Some(mut s) => {
                s.insert(var.to_string(), val);
                Some(s)
            }
            None => panic!("[ERROR] bottom state"),
        }
    }
}

impl Lattice for State {
    fn union(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.union(b))
    }

    fn intersection(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.intersection(b))
    }

    fn widen(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.widen(b))
    }

    fn narrow(&self, other: Self) -> Self {
        point_wise_op(&self, &other, |a, b| a.narrow(b))
    }
}

// --- helpers

pub fn empty_state() -> State {
    Some(HashMap::new())
}

pub fn default_invariant() -> Invariant {
    vec![]
}

pub fn concat(invariants: &[Invariant]) -> Invariant {
    let mut final_inv = vec![];
    for inv in invariants {
        final_inv.extend(inv.clone());
    }
    final_inv
}

fn point_wise_op(s1: &State, s2: &State, op: fn(Interval, Interval) -> Interval) -> State {
    match (s1, s2) {
        (Some(s1), Some(s2)) => {
            let mut new_state = empty_state();
            for (var1, val1) in s1 {
                match s2.get(var1) {
                    Some(val2) => new_state = new_state.put(&var1, op(*val1, *val2)),
                    None => new_state = new_state.put(&var1, *val1),
                }
            }
            new_state
        }
        _ => None,
    }
}
