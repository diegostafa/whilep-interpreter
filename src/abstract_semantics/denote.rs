use crate::abstract_semantics::invariant::*;
use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

// --- type aliases

pub type StateFunction<'a, T> =
    Box<dyn Fn((State<T>, Invariant<T>)) -> (State<T>, Invariant<T>) + 'a>;

pub type StateTransformer<'a, T> =
    Box<dyn Fn(&State<T>) -> (State<T>, Vec<State<T>>, State<T>) + 'a>;

// --- ast denotation

pub fn denote_stmt<'a, T: Domain + 'a>(stmt: Statement) -> StateFunction<'a, T> {
    match stmt {
        Statement::Skip => id(),
        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),
        Statement::While { cond, body } => lfp(*cond, denote_stmt(*body)),
    }
}

// --- semantic functions

fn id<'a, T: Domain + 'a>() -> StateFunction<'a, T> {
    Box::new(|(state, inv)| (state.clone(), inv.append(state)))
}

fn compose<'a, T: Domain + 'a>(
    f: StateFunction<'a, T>,
    g: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |(state, inv)| g(f((state, inv))))
}

fn state_update<'a, T: Domain + 'a>(var: String, val: ArithmeticExpr) -> StateFunction<'a, T> {
    Box::new(move |(state, inv)| {
        let (interval, new_state) = T::eval_aexpr(&val, &state);
        let new_state = new_state.put(&var, interval);
        (new_state.clone(), inv.append(new_state))
    })
}

fn conditional<'a, T: Domain + 'a>(
    cond: BooleanExpr,
    s1: StateFunction<'a, T>,
    s2: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |(state, inv)| {
        let if_cond_state = T::eval_bexpr(&cond, &state);
        let (s1_state, s1_inv) = s1((if_cond_state.clone(), Invariant::new()));

        let el_cond_state = T::eval_bexpr(&negate_bexpr(&cond), &state);
        let (s2_state, s2_inv) = s2((el_cond_state.clone(), Invariant::new()));

        let end_state = s1_state.union(&s2_state);
        (
            end_state.clone(),
            inv.concat(&[
                vec![if_cond_state],
                s1_inv,
                vec![el_cond_state],
                s2_inv,
                vec![end_state],
            ]),
        )
    })
}

fn lfp<'a, T: Domain + 'a>(cond: BooleanExpr, body: StateFunction<'a, T>) -> StateFunction<'a, T> {
    Box::new(move |(state, inv)| {
        let iteration = Box::new(|s: &State<T>| {
            let cond_state = T::eval_bexpr(&cond, &s);
            let (body_state, body_inv) = body((cond_state.clone(), Invariant::new()));
            let done_state = state.widen(&body_state);
            (cond_state, body_inv, done_state)
        });

        let fix = |f: StateTransformer<T>| {
            let mut prev_state = state.clone();
            loop {
                let (cond_state, body_inv, done_state) = f(&prev_state);
                match prev_state == done_state {
                    true => break (cond_state, body_inv, done_state),
                    _ => prev_state = done_state,
                }
            }
        };

        let (cond_state, body_inv, done_state) = fix(iteration);
        let done_state_narrowed = done_state.narrow(&body_inv.back());
        let end_state = T::eval_bexpr(&negate_bexpr(&cond), &done_state_narrowed);

        (
            end_state.clone(),
            inv.concat(&[vec![cond_state], body_inv, vec![end_state]]),
        )
    })
}
