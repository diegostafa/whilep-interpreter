use crate::ast::ArithmeticExpr;
use crate::integer::Integer;
use crate::interval::*;
use crate::lattice::Lattice;
use crate::pretty_print::PrettyPrint;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Bottom,
    Just(HashMap<String, Interval>),
}

pub type Invariant = Vec<State>;

pub trait IO {
    fn read(&self, var: &String) -> Interval;
    fn put(&self, var: &String, val: Interval) -> Self;
    fn try_put(&self, var: &ArithmeticExpr, val: Interval) -> Self;
}

impl IO for State {
    fn read(&self, var: &String) -> Interval {
        match self {
            State::Just(state) => match state.get(var) {
                Some(val) => *val,
                None => Interval::Range(Integer::NegInf, Integer::PosInf),
            },
            State::Bottom => panic!("[ERROR] bottom state"),
        }
    }

    fn put(&self, var: &String, val: Interval) -> Self {
        match self.clone() {
            State::Just(mut s) => match val {
                Interval::Bottom => State::Bottom,
                _ => {
                    s.insert(var.to_string(), val);
                    State::Just(s)
                }
            },
            State::Bottom => panic!("[ERROR] bottom state"),
        }
    }

    fn try_put(&self, var: &ArithmeticExpr, val: Interval) -> Self {
        match (self.clone(), var) {
            (State::Just(mut s), ArithmeticExpr::Identifier(var)) => {
                s.insert(var.to_string(), val);
                State::Just(s)
            }
            (State::Just(s), _) => State::Just(s),
            _ => State::Bottom,
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
    State::Just(HashMap::new())
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
        (State::Just(s1), State::Just(s2)) => {
            let mut new_state = empty_state();
            for (var1, val1) in s1 {
                match s2.get(var1) {
                    Some(val2) => new_state = new_state.put(&var1, op(*val1, *val2)),
                    None => new_state = new_state.put(&var1, *val1),
                }
            }
            new_state
        }
        _ => State::Bottom,
    }
}

impl PrettyPrint for State {
    fn pretty_print(&self) {
        match self {
            State::Just(s) => {
                let mut pretty_state = s
                    .iter()
                    .map(|(var, val)| format!("{}: {}", var, val))
                    .collect::<Vec<String>>();

                pretty_state.sort();
                println!("\t{}", pretty_state.join(", "))
            }
            State::Bottom => println!("\tBOTTOM STATE"),
        }
    }
}

impl PrettyPrint for Invariant {
    fn pretty_print(&self) {
        for state in self {
            state.pretty_print();
        }
    }
}
