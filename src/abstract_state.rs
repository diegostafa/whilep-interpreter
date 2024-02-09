use crate::ast::ArithmeticExpr;
use crate::integer::Integer;
use crate::interval::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq)]
pub enum State {
    Bottom,
    Just(HashMap<String, Interval>),
}

impl State {
    pub fn read(&self, var: &String) -> Interval {
        match self {
            State::Just(state) => match state.get(var) {
                Some(val) => *val,
                None => Interval::Range(Integer::NegInf, Integer::PosInf),
            },
            State::Bottom => panic!("[ERROR] bottom state"),
        }
    }

    pub fn put(&self, var: &String, val: Interval) -> Self {
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

    pub fn try_put(&self, var: &ArithmeticExpr, val: Interval) -> Self {
        match (self.clone(), var) {
            (State::Just(mut s), ArithmeticExpr::Identifier(var)) => {
                s.insert(var.to_string(), val);
                State::Just(s)
            }
            (State::Just(s), _) => State::Just(s),
            _ => State::Bottom,
        }
    }

    pub fn union(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.union(b))
    }

    pub fn intersection(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.intersection(b))
    }

    pub fn widen(&self, other: Self) -> Self {
        point_wise_op(self, &other, |a, b| a.widen(b))
    }

    pub fn narrow(&self, other: Self) -> Self {
        point_wise_op(&self, &other, |a, b| a.narrow(b))
    }

    pub fn pretty_print(&self) {
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Bottom, State::Bottom) => true,
            (State::Bottom, _) | (_, State::Bottom) => false,
            (State::Just(s1), State::Just(s2)) => {
                if s1.len() != s2.len() {
                    return false;
                }

                for (k, v1) in s1.iter() {
                    if let Some(v2) = s2.get(k) {
                        if v1 != v2 {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    }
}

// --- invariants
pub type Invariant = Vec<State>;

pub fn default_invariant() -> Invariant {
    vec![]
}

pub fn merge_invariants(invariants: &[Invariant]) -> Invariant {
    let mut final_inv = vec![];
    for inv in invariants {
        final_inv.extend(inv.clone());
    }
    final_inv
}

pub fn pretty_print_inv(invariants: &Invariant) {
    for state in invariants {
        state.pretty_print();
    }
}

// --- helpers

pub fn empty_state() -> State {
    State::Just(HashMap::new())
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
