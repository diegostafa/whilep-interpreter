use crate::abstract_semantics::invariant::*;
use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

// --- type aliases

pub type StateFunction<'a, T> = Box<dyn Fn(State<T>) -> (State<T>, Invariant<T>) + 'a>;
pub type StateTransformer<'a, T> =
    Box<dyn Fn(&State<T>) -> (State<T>, Vec<State<T>>, State<T>) + 'a>;

// --- ast denotation

pub fn denote_stmt<'a, T: Domain + 'a>(stmt: Statement) -> StateFunction<'a, T> {
    match stmt {
        Statement::Skip => id(),
        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),
        Statement::While { cond, body } => fix_while(*cond, denote_stmt(*body)),
        Statement::RepeatUntil { body, cond } => fix_repeat(*cond, denote_stmt(*body)),
    }
}

// --- semantic functions

fn id<'a, T: Domain + 'a>() -> StateFunction<'a, T> {
    Box::new(|state| (state.clone(), vec![state]))
}

fn compose<'a, T: Domain + 'a>(
    f: StateFunction<'a, T>,
    g: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let (f_state, f_inv) = f(state);
        let (g_state, g_inv) = g(f_state);
        (g_state, concat(&[f_inv, g_inv]))
    })
}

fn state_update<'a, T: Domain + 'a>(var: String, val: ArithmeticExpr) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let (interval, new_state) = T::eval_aexpr(&val, &state);
        let new_state = new_state.put(&var, interval);
        (new_state.clone(), vec![new_state])
    })
}

fn conditional<'a, T: Domain + 'a>(
    cond: BooleanExpr,
    s1: StateFunction<'a, T>,
    s2: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let if_state = T::eval_bexpr(&cond, &state);
        let el_state = T::eval_bexpr(&negate_bexpr(&cond), &state);
        let (s1_state, s1_inv) = s1(if_state.clone());
        let (s2_state, s2_inv) = s2(el_state.clone());
        let end_state = s1_state.union(&s2_state);
        (
            end_state.clone(),
            concat(&[
                vec![if_state],
                s1_inv,
                vec![el_state],
                s2_inv,
                vec![end_state],
            ]),
        )
    })
}

fn fix_while<'a, T: Domain + 'a>(
    cond: BooleanExpr,
    body: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let iteration = Box::new(|prev_state: &State<T>| {
            let cond_state = T::eval_bexpr(&cond, &prev_state);
            let (body_state, body_inv) = body(cond_state.clone());
            let exit_state = state.widen(&body_state);
            (cond_state, body_inv, exit_state)
        });

        let lfp = |f: StateTransformer<T>| {
            let mut prev_state = State::Bottom;
            loop {
                let (cond_state, body_inv, exit_state) = f(&prev_state);
                match prev_state == exit_state {
                    true => break (cond_state, body_inv, exit_state),
                    _ => prev_state = exit_state,
                }
            }
        };

        let (cond_state, body_inv, exit_state) = lfp(iteration);

        let exit_state = match body_inv.back() {
            State::Bottom => exit_state,
            s => exit_state.narrow(&s),
        };

        let exit_state = T::eval_bexpr(&negate_bexpr(&cond), &exit_state);
        (
            exit_state.clone(),
            concat(&[vec![cond_state], body_inv, vec![exit_state]]),
        )
    })
}

fn fix_repeat<'a, T: Domain + 'a>(
    cond: BooleanExpr,
    body: StateFunction<'a, T>,
) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let one_step_state = body(state.clone()).0;

        let iteration = Box::new(|prev_state: &State<T>| {
            let cond_state = T::eval_bexpr(&negate_bexpr(&cond), &prev_state);
            let (body_state, body_inv) = body(cond_state.clone());
            let cond_state = T::eval_bexpr(&negate_bexpr(&cond), &body_state);
            let exit_state = one_step_state.widen(&body_state);
            (cond_state, body_inv, exit_state)
        });

        let lfp = |f: StateTransformer<T>| {
            let mut prev_state = State::Bottom;

            loop {
                let (cond_state, body_inv, exit_state) = f(&prev_state);
                match prev_state == exit_state {
                    true => break (cond_state, body_inv, exit_state),
                    _ => prev_state = exit_state,
                }
            }
        };

        let (cond_state, body_inv, exit_state) = lfp(iteration);

        let exit_state = match body_inv.back() {
            State::Bottom => one_step_state.clone(),
            s => exit_state.narrow(&s),
        };

        let exit_state = T::eval_bexpr(&cond, &exit_state);
        (
            exit_state.clone(),
            concat(&[
                vec![one_step_state],
                body_inv,
                vec![cond_state],
                vec![exit_state],
            ]),
        )
    })
}
