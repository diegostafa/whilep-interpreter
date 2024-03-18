use crate::abstract_semantics::invariant::*;
use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

// --- type aliases

pub type StateFunction<'a, T> = Box<dyn Fn(State<T>) -> (State<T>, Invariant<T>) + 'a>;
pub type LoopIteration<'a, T> = Box<dyn Fn(&State<T>) -> State<T> + 'a>;

// --- ast denotation

pub fn denote_stmt<'a, T: Domain + 'a>(stmt: Statement) -> StateFunction<'a, T> {
    match stmt.clone() {
        Statement::Skip => id(),

        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),

        Statement::Assignment { var, val } => state_update(var, *val),

        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),

        Statement::While { cond, body, delay } => {
            let body = denote_stmt(*body);

            Box::new(move |state| {
                let f: LoopIteration<T> = Box::new(|prev_state: &State<T>| {
                    let cond_state = T::eval_bexpr(&cond, &prev_state);
                    let body_state = body(cond_state).0;
                    state.lub(&body_state)
                });

                while_semantic(
                    f,
                    &cond,
                    &body,
                    delay.unwrap_or(stmt.get_max_number().unwrap_or(0)),
                )
            })
        }

        Statement::RepeatUntil { body, cond, delay } => denote_stmt(Statement::Chain(
            body.clone(),
            Box::new(Statement::While {
                cond: Box::new(cond.negate()),
                body: Box::new(*body),
                delay: delay,
            }),
        )),
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

fn state_update<'a, T: Domain + 'a>(var: Identifier, val: ArithmeticExpr) -> StateFunction<'a, T> {
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
        let el_state = T::eval_bexpr(&cond.negate(), &state);
        let (s1_state, s1_inv) = s1(if_state.clone());
        let (s2_state, s2_inv) = s2(el_state.clone());
        let end_state = s1_state.lub(&s2_state);
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

fn while_semantic<T: Domain>(
    f: LoopIteration<T>,
    cond: &BooleanExpr,
    body: &StateFunction<T>,
    delay: i64,
) -> (State<T>, Invariant<T>) {
    let loop_inv = fix_wide(&f, State::Bottom, delay);
    let loop_inv = fix_narr(&f, loop_inv);

    let cond_state = T::eval_bexpr(&cond, &loop_inv);
    let body_state = body(cond_state.clone()).1;
    let exit_state = T::eval_bexpr(&cond.negate(), &loop_inv);

    (
        exit_state.clone(),
        concat(&[
            vec![loop_inv],
            vec![cond_state],
            body_state,
            vec![exit_state],
        ]),
    )
}

fn fix_wide<T: Domain>(f: &LoopIteration<T>, init_state: State<T>, mut delay: i64) -> State<T> {
    let mut prev_state = init_state;
    loop {
        let mut curr_state = f(&prev_state);

        if delay == 0 {
            curr_state = prev_state.widen(&curr_state);
        } else {
            delay -= 1;
        }

        if prev_state == curr_state {
            break curr_state;
        }

        prev_state = curr_state;
    }
}

fn fix_narr<T: Domain>(f: &LoopIteration<T>, init_state: State<T>) -> State<T> {
    let mut prev_state = init_state;
    loop {
        let mut curr_state = f(&prev_state);
        curr_state = prev_state.narrow(&curr_state);

        if prev_state == curr_state {
            break curr_state;
        }

        prev_state = curr_state;
    }
}
