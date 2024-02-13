use std::collections::HashMap;
use std::fmt;

use crate::domain::domain::*;
use crate::domain::expression_tree::*;
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
            State::Bottom => T::BOT,
            State::Just(state) => match state.get(var) {
                Some(val) => val.clone(),
                None => T::TOP,
            },
        }
    }

    pub fn put(&self, var: &String, val: T) -> Self {
        match self.clone() {
            State::Bottom => State::Bottom,
            _ if val == T::BOT => State::Bottom,
            State::Just(mut s) => {
                s.insert(var.to_string(), val);
                State::Just(s)
            }
        }
    }

    pub fn refine_expression_tree(&self, tree: &ExpressionTree<T>, refined_value: T) -> Self {
        match tree {
            ExpressionTree::Value(_) => self.clone(),
            ExpressionTree::Variable(var, val) => self.put(var, val.intersection(&refined_value)),
            ExpressionTree::Binop(op, val, l, r) => {
                let c = val.intersection(&refined_value);
                let a = l.get_value();
                let b = r.get_value();

                let (lval, rval) = match op {
                    ArithmeticExpr::Add(_, _) => (c.clone() - b, c - a),
                    ArithmeticExpr::Sub(_, _) => (c.clone() + b, a - c),
                    ArithmeticExpr::Mul(_, _) => (c.clone() / b, c / a),
                    ArithmeticExpr::Div(_, _) => (c.clone() * b, a / c),
                    _ => unreachable!(),
                };

                self.refine_expression_tree(l, lval)
                    .refine_expression_tree(r, rval)
            }
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

impl<T: Domain> fmt::Display for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Bottom => write!(f, "BOTTOM STATE"),
            State::Just(s) if s.is_empty() => write!(f, "-"),
            State::Just(s) => {
                let mut pretty_state = s
                    .iter()
                    .map(|(var, val)| format!("{}: {}", var, val))
                    .collect::<Vec<String>>();

                pretty_state.sort();
                write!(f, "{}", pretty_state.join(", "))
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
