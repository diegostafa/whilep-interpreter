use std::collections::HashMap;
use std::fmt::Display;

use crate::{ast::*, domain::*, interval::*, invariant::*, lattice::*};

#[derive(Debug, Clone, Eq)]
pub enum State<T: Domain + Lattice + Display + Clone + Eq> {
    Bottom,
    Just(HashMap<String, T>),
}

impl<T: Domain + Lattice + Display + Clone + Eq> State<T> {
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

    pub fn eval_stmt(
        ast: Statement,
        (state, inv): (Self, Invariant<T>),
        bounds: Interval,
    ) -> (Self, Invariant<T>) {
        match state {
            State::Bottom => (state, inv),
            _ => match ast {
                Statement::Skip => (state.clone(), inv.append(&[vec![state.clone()]])),

                Statement::Chain(s1, s2) => {
                    State::eval_stmt(*s2, State::eval_stmt(*s1, (state, inv), bounds), bounds)
                }
                Statement::Assignment { var, val } => {
                    let (interval, new_state) = Domain::eval_aexpr(&val, &state);
                    let new_state = new_state.put(&var, interval);
                    (new_state.clone(), inv.append(&[vec![new_state]]))
                }
                Statement::If { cond, s1, s2 } => {
                    let tt_state = Domain::eval_bexpr(&cond, &state);
                    let ff_state = Domain::eval_bexpr(&negate_bexpr(&cond), &state);
                    let (s1_state, s1_inv) =
                        State::eval_stmt(*s1, (tt_state.clone(), Invariant::new()), bounds);
                    let (s2_state, s2_inv) =
                        State::eval_stmt(*s2, (ff_state.clone(), Invariant::new()), bounds);
                    let endif_state = s1_state.union(s2_state);
                    (
                        endif_state.clone(),
                        inv.append(&[
                            vec![tt_state],
                            s1_inv,
                            vec![ff_state],
                            s2_inv,
                            vec![endif_state],
                        ]),
                    )
                }
                Statement::While { cond, body } => {
                    let mut pre_state = state;

                    let (post_state, cond_inv, body_inv) = loop {
                        let cond_state = Domain::eval_bexpr(&cond, &pre_state);
                        let (body_state, body_inv) = State::eval_stmt(
                            *body.clone(),
                            (cond_state.clone(), Invariant::new()),
                            bounds,
                        );

                        let post_state = cond_state.widen(body_state);
                        match pre_state == post_state {
                            true => break (post_state, vec![cond_state], body_inv),
                            _ => pre_state = post_state,
                        }
                    };

                    let narrowed_state = post_state.narrow(body_inv.last().unwrap().clone());
                    let exit_state = Domain::eval_bexpr(&negate_bexpr(&cond), &narrowed_state);

                    body_inv.pretty_print();
                    exit_state.pretty_print();

                    (
                        narrowed_state,
                        inv.append(&[cond_inv, body_inv, vec![exit_state]]),
                    )
                }
            },
        }
    }
}

impl<T: Domain + Display + Clone + Eq> Lattice for State<T> {
    const TOP: Self = State::Bottom;
    const BOT: Self = State::Bottom;

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

impl<T: Domain + Lattice + Display + Clone + Eq + Eq> PartialEq for State<T> {
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

fn point_wise_op<T: Domain + Lattice + Display + Clone + Eq>(
    s1: &State<T>,
    s2: &State<T>,
    op: fn(T, T) -> T,
) -> State<T> {
    match (s1, s2) {
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
        _ => State::Bottom,
    }
}
