use crate::domain::domain::*;
use crate::domain::expression_tree::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;
use std::collections::HashMap;
use std::fmt;

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
            State::Just(state) => *state.get(var).unwrap_or(&T::TOP),
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
    const UNIT: Self = State::Bottom;

    fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (State::Bottom, _) => other.clone(),
            (_, State::Bottom) => self.clone(),
            (State::Just(s1), State::Just(s2)) => point_wise_op(s1, s2, |a, b| a.union(&b)),
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        match (self, other) {
            (State::Bottom, _) | (_, State::Bottom) => State::Bottom,
            (State::Just(s1), State::Just(s2)) => point_wise_op(s1, s2, |a, b| a.intersection(&b)),
        }
    }

    fn widen(&self, other: &Self) -> Self {
        match (self, other) {
            (State::Bottom, _) => other.clone(),
            (_, State::Bottom) => self.clone(),
            (State::Just(s1), State::Just(s2)) => point_wise_op(s1, s2, |a, b| a.widen(&b)),
        }
    }

    fn narrow(&self, other: &Self) -> Self {
        match (self, other) {
            (State::Bottom, _) | (_, State::Bottom) => State::Bottom,
            (State::Just(s1), State::Just(s2)) => point_wise_op(s1, s2, |a, b| a.narrow(&b)),
        }
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
            State::Just(s) if s.is_empty() => write!(f, "EMPTY STATE"),
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

fn point_wise_op<T: Domain>(
    s1: &HashMap<String, T>,
    s2: &HashMap<String, T>,
    op: fn(T, T) -> T,
) -> State<T> {
    let mut new_state = State::new();
    for (var1, val1) in s1 {
        match s2.get(var1) {
            Some(val2) => new_state = new_state.put(&var1, op(val1.clone(), val2.clone())),
            None => new_state = new_state.put(&var1, val1.clone()),
        }
    }
    new_state
}
