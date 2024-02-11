use std::collections::HashMap;

use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

#[derive(Debug, Clone, Eq)]
pub enum State<T: Domain> {
    Bottom,
    Just(HashMap<String, T>),
}

impl<T: Domain> State<T> {
    pub fn new() -> Self {
        State::Just(HashMap::new())
    }

    pub fn read(&self, var: &String) -> T {
        match self {
            State::Just(state) => match state.get(var) {
                Some(val) => val.clone(),
                None => T::TOP,
            },
            State::Bottom => panic!("[ERROR] bottom state"),
        }
    }

    pub fn put(&self, var: &String, val: T) -> Self {
        match self.clone() {
            State::Just(mut s) => match val == T::BOT {
                true => State::Bottom,
                _ => {
                    s.insert(var.to_string(), val);
                    State::Just(s)
                }
            },
            State::Bottom => panic!("[ERROR] bottom state"),
        }
    }

    pub fn try_put(&self, var: &ArithmeticExpr, val: T) -> Self {
        match (self.clone(), var) {
            (State::Just(mut s), ArithmeticExpr::Identifier(var)) => {
                s.insert(var.to_string(), val);
                State::Just(s)
            }
            (State::Just(_), _) => self.clone(),
            _ => State::Bottom,
        }
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

impl<T: Domain> Lattice for State<T> {
    const TOP: Self = State::Bottom;
    const BOT: Self = State::Bottom;

    fn union(&self, other: &Self) -> Self {
        point_wise_op(self, &other, |a, b| a.union(&b))
    }

    fn intersection(&self, other: &Self) -> Self {
        point_wise_op(self, &other, |a, b| a.intersection(&b))
    }

    fn widen(&self, other: &Self) -> Self {
        point_wise_op(self, &other, |a, b| a.widen(&b))
    }

    fn narrow(&self, other: &Self) -> Self {
        point_wise_op(&self, &other, |a, b| a.narrow(&b))
    }
}

impl<T: Domain> PartialEq for State<T> {
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

// --- helpers

fn point_wise_op<T: Domain>(s1: &State<T>, s2: &State<T>, op: fn(T, T) -> T) -> State<T> {
    match (s1, s2) {
        (State::Bottom, _) => s2.clone(),
        (_, State::Bottom) => s1.clone(),
        (State::Just(s1), State::Just(s2)) => {
            let mut new_state = State::new();
            for (var1, val1) in s1 {
                match s2.get(var1) {
                    Some(val2) => new_state = new_state.put(&var1, op(val1.clone(), val2.clone())),
                    None => new_state = new_state.put(&var1, val1.clone()),
                }
            }
            new_state
        }
    }
}

// --- ALTERNATIVE IMPLEMANTION
/*
 pub fn eval_stmt(&self, ast: &Statement, inv: &Invariant<T>) -> (State<T>, Invariant<T>) {
        let state = self.clone();
        match state {
            State::Bottom => (state.clone(), inv.clone()),
            _ => match ast {
                Statement::Skip => (state.clone(), inv.append(&[vec![state.clone()]])),

                Statement::Chain(s1, s2) => {
                    let (new_state, new_inv) = state.eval_stmt(s1, inv);
                    new_state.eval_stmt(s2, &new_inv)
                }

                Statement::Assignment { var, val } => {
                    let (interval, new_state) = T::eval_aexpr(&val, &state);
                    let new_state = new_state.put(&var, interval);
                    (new_state.clone(), inv.append(&[vec![new_state]]))
                }

                Statement::If { cond, s1, s2 } => {
                    let tt_state = T::eval_bexpr(&cond, &state);
                    let ff_state = T::eval_bexpr(&negate_bexpr(&cond), &state);
                    let (s1_state, s1_inv) = tt_state.eval_stmt(s1, &Invariant::new());
                    let (s2_state, s2_inv) = ff_state.eval_stmt(s2, &Invariant::new());

                    let end_state = s1_state.union(&s2_state);
                    (
                        end_state.clone(),
                        inv.append(&[
                            vec![tt_state],
                            s1_inv,
                            vec![ff_state],
                            s2_inv,
                            vec![end_state],
                        ]),
                    )
                }

                Statement::While { cond, body } => {
                    let mut prev_state: State<T> = state.clone();

                    let (post_state, cond_inv, body_inv) = loop {
                        let cond_state = T::eval_bexpr(&cond, &prev_state);
                        let body_eval = cond_state.eval_stmt(body, &Invariant::new());
                        let curr_state = cond_state.widen(&body_eval.0);

                        match prev_state == curr_state {
                            true => break (curr_state, vec![cond_state], body_eval.1),
                            _ => prev_state = curr_state,
                        }
                    };

                    let narrowed_state = post_state.narrow(body_inv.last().unwrap());
                    let end_state = T::eval_bexpr(&negate_bexpr(&cond), &narrowed_state);

                    (
                        narrowed_state,
                        inv.append(&[cond_inv, body_inv, vec![end_state]]),
                    )
                }
            },
        }
    }
*/