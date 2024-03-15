use crate::abstract_semantics::invariant::*;
use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::domain::lattice::*;
use crate::parser::ast::*;

// --- type aliases

pub type StateFunction<'a, T> = Box<dyn Fn(State<T>) -> (State<T>, Invariant<T>) + 'a>;
pub type LoopIteration<'a, T> = Box<dyn Fn(&State<T>) -> (State<T>, Invariant<T>) + 'a>;

// --- ast denotation

pub fn denote_stmt<'a, T: Domain + 'a>(stmt: Statement) -> StateFunction<'a, T> {
    match stmt.clone() {
        Statement::Skip => id(),
        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),
        Statement::While { cond, body, delay } => fix_while(
            *cond.clone(),
            denote_stmt(*body),
            delay.unwrap_or(stmt.get_max_number_or(0)),
        ),
        Statement::RepeatUntil {
            body: _,
            cond: _,
            delay: _,
        } => todo!(),
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
    delay: i64,
) -> StateFunction<'a, T> {
    Box::new(move |state| {
        let f = Box::new(|prev_state: &State<T>| {
            let cond_state = T::eval_bexpr(&cond, &prev_state);
            let (body_state, body_inv) = body(cond_state);
            (state.union(&body_state), body_inv)
        });

        let (loop_inv, _) = fix_wide(f.clone(), State::Bottom, delay);
        let (loop_inv, body_inv) = fix_narr(f, loop_inv, delay);

        let cond_state = T::eval_bexpr(&cond, &loop_inv);
        let post_cond = T::eval_bexpr(&cond.negate(), &loop_inv);

        (
            post_cond.clone(),
            concat(&[vec![loop_inv], vec![cond_state], body_inv, vec![post_cond]]),
        )
    })
}

fn fix_wide<T: Domain>(
    f: LoopIteration<T>,
    mut prev_state: State<T>,
    mut delay: i64,
) -> (State<T>, Invariant<T>) {
    loop {
        let (mut curr_state, inv) = f(&prev_state);

        if delay <= 0 {
            curr_state = prev_state.widen(&curr_state);
        } else {
            delay -= 1;
        }

        if prev_state == curr_state {
            break (curr_state, inv);
        }

        prev_state = curr_state;
    }
}

fn fix_narr<T: Domain>(
    f: LoopIteration<T>,
    mut prev_state: State<T>,
    mut delay: i64,
) -> (State<T>, Invariant<T>) {
    loop {
        let (mut curr_state, inv) = f(&prev_state);

        if delay <= 0 {
            curr_state = prev_state.narrow(&curr_state);
        } else {
            delay -= 1;
        }

        if prev_state == curr_state {
            break (curr_state, inv);
        }

        prev_state = curr_state;
    }
}
