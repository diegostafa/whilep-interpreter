use crate::abstract_state::*;
use crate::ast::*;
use crate::integer::Integer;
use crate::interval::*;
use crate::lattice::*;

fn g(cond: BooleanExpr, body: Statement, (state, inv): (State, Invariant)) -> (State, Invariant) {
    let (new_s, new_i) = eval_stmt(body, (eval_bexpr(&cond.clone(), &state), inv));
    (state.union(new_s), new_i)
}

pub fn eval_stmt(ast: Statement, (state, inv): (State, Invariant)) -> (State, Invariant) {
    match ast {
        Statement::Skip => (state.clone(), concat(&[inv, vec![(state)]])),

        Statement::Assignment { var, val } => {
            let (interval, new_state) = eval_aexpr(&val, &state);
            let new_state = new_state.put(&var, interval);
            (new_state.clone(), concat(&[inv, vec![new_state]]))
        }

        Statement::Chain(s1, s2) => eval_stmt(*s2, eval_stmt(*s1, (state, inv))),

        Statement::If { cond, s1, s2 } => {
            let tt_state = eval_bexpr(&cond, &state);
            let ff_state = eval_bexpr(&negate_bexpr(&cond), &state);
            let (s1_state, s1_inv) = eval_stmt(*s1, (tt_state.clone(), default_invariant()));
            let (s2_state, s2_inv) = eval_stmt(*s2, (ff_state.clone(), default_invariant()));
            (
                s1_state.union(s2_state),
                concat(&[inv, vec![tt_state], s1_inv, vec![ff_state], s2_inv]),
            )
        }

        Statement::While { cond, body } => {
            let mut new_s = State::Bottom;
            let mut new_inv = inv.clone();
            while new_s == State::Bottom {
                (new_s, new_inv) = g(*cond.clone(), *body.clone(), (state.clone(), inv.clone()));
            }

            (eval_bexpr(&negate_bexpr(&cond), &new_s), new_inv)
        }
    }
}

pub fn eval_aexpr(expr: &ArithmeticExpr, state: &State) -> (Interval, State) {
    match expr {
        ArithmeticExpr::Number(n) => (Interval::Range(*n, *n), state.clone()),
        ArithmeticExpr::Interval(n, m) => (Interval::Range(*n, *m), state.clone()),
        ArithmeticExpr::Identifier(var) => (state.read(var), state.clone()),
        ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
        ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
        ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
        ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),
        ArithmeticExpr::PostIncrement(var) => {
            let interval = state.read(var);
            (interval, state.put(var, interval.shift(Integer::Value(1))))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let interval = state.read(var);
            (interval, state.put(var, interval.shift(Integer::Value(-1))))
        }
    }
}

pub fn eval_bexpr(expr: &BooleanExpr, state: &State) -> State {
    match expr {
        BooleanExpr::True => state.clone(),
        BooleanExpr::False => State::Bottom,
        BooleanExpr::Not(b) => eval_bexpr(&desugar_not_bexpr(*b.clone()), state),
        BooleanExpr::And(b1, b2) => eval_bexpr(b1, state).intersection(eval_bexpr(b2, state)),
        BooleanExpr::Or(b1, b2) => eval_bexpr(b1, state).union(eval_bexpr(b2, state)),
        BooleanExpr::NumEq(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.intersection(a2_interval) {
                Interval::Bottom => State::Bottom,
                intersection => new_state
                    .try_put(a1, intersection)
                    .try_put(a2, intersection),
            }
        }
        BooleanExpr::NumNotEq(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.intersection(a2_interval) {
                Interval::Bottom => new_state,
                intersection => new_state
                    .try_put(a1, a1_interval.difference(intersection))
                    .try_put(a2, a2_interval.difference(intersection)),
            }
        }
        BooleanExpr::NumLt(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.on_right(a2_interval) {
                true => State::Bottom,
                _ => new_state
                    .try_put(a1, a1_interval.clamp_max(a2_interval))
                    .try_put(a2, a2_interval.clamp_min(a1_interval)),
            }
        }
        BooleanExpr::NumGt(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.on_left(a2_interval) {
                true => State::Bottom,
                _ => new_state
                    .try_put(a1, a1_interval.clamp_min(a2_interval))
                    .try_put(a2, a2_interval.clamp_max(a1_interval)),
            }
        }
        BooleanExpr::NumLtEq(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.on_right(a2_interval) && !a1_interval.overlaps(a2_interval) {
                true => State::Bottom,
                _ => new_state
                    .try_put(a1, a1_interval.clamp_max(a2_interval))
                    .try_put(a2, a2_interval.clamp_min(a1_interval)),
            }
        }
        BooleanExpr::NumGtEq(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match a1_interval.on_left(a2_interval) && !a1_interval.overlaps(a2_interval) {
                true => State::Bottom,
                _ => new_state
                    .try_put(a1, a1_interval.clamp_max(a2_interval))
                    .try_put(a2, a2_interval.clamp_min(a1_interval)),
            }
        }
    }
}

// --- helpers

fn trans_aexpr(
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, Interval, State) {
    let (a1_interval, new_state) = eval_aexpr(a1, &state);
    let (a2_interval, new_state) = eval_aexpr(a2, &new_state);
    (a1_interval, a2_interval, new_state)
}

fn binop_aexpr(
    op: fn(Interval, Interval) -> Interval,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, State) {
    let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
    (op(a1_interval, a2_interval), new_state)
}
