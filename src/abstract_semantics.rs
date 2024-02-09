use crate::abstract_state::*;
use crate::ast::*;
use crate::integer::Integer;
use crate::interval::*;

pub fn eval_stmt(ast: Statement, (state, inv): (State, Invariant)) -> (State, Invariant) {
    match state {
        State::Bottom => (state, inv),
        _ => match ast {
            Statement::Skip => (state.clone(), merge_invariants(&[inv, vec![state]])),
            Statement::Chain(s1, s2) => eval_stmt(*s2, eval_stmt(*s1, (state, inv))),
            Statement::Assignment { var, val } => {
                let (interval, new_state) = eval_aexpr(&val, &state);
                let new_state = new_state.put(&var, interval);
                (new_state.clone(), merge_invariants(&[inv, vec![new_state]]))
            }
            Statement::If { cond, s1, s2 } => {
                let tt_state = eval_bexpr(&cond, &state);
                let ff_state = eval_bexpr(&negate_bexpr(&cond), &state);
                let (s1_state, s1_inv) = eval_stmt(*s1, (tt_state.clone(), default_invariant()));
                let (s2_state, s2_inv) = eval_stmt(*s2, (ff_state.clone(), default_invariant()));
                let endif_state = s1_state.union(s2_state);
                (
                    endif_state.clone(),
                    merge_invariants(&[
                        inv,
                        vec![tt_state],
                        s1_inv,
                        vec![ff_state],
                        s2_inv,
                        vec![endif_state],
                    ]),
                )
            }
            Statement::While { cond, body } => {
                let mut pre_state = state.clone();

                let (post_state, cond_inv, body_inv) = loop {
                    let cond_state = eval_bexpr(&cond, &pre_state);

                    let (body_state, body_inv) =
                        eval_stmt(*body.clone(), (cond_state.clone(), default_invariant()));

                    let post_state = cond_state.widen(body_state);
                    match pre_state == post_state {
                        true => break (post_state, vec![cond_state], body_inv),
                        false => pre_state = post_state.clone(),
                    }
                };

                let narrowed_state = post_state.narrow(body_inv.last().unwrap().clone());
                let exit_state = eval_bexpr(&negate_bexpr(&cond), &narrowed_state);

                pretty_print_inv(&body_inv);
                exit_state.pretty_print();

                (
                    narrowed_state,
                    merge_invariants(&[inv, cond_inv, body_inv, vec![exit_state]]),
                )
            }
        },
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
            let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
            match i1.intersection(i2) {
                Interval::Bottom => State::Bottom,
                intersection => new_state
                    .try_put(a1, intersection)
                    .try_put(a2, intersection),
            }
        }
        BooleanExpr::NumNotEq(a1, a2) => {
            let (_, _, new_state) = trans_aexpr(a1, a2, &state);
            new_state
        }

        BooleanExpr::NumLt(a1, a2) => {
            let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
            let one = Integer::Value(1);
            let lhs = i1.intersection(i2.add_min(Integer::NegInf).add_max(-one));
            let rhs = i2.intersection(i1.add_max(Integer::PosInf).add_min(one));
            match (lhs, rhs) {
                (Interval::Bottom, _) | (_, Interval::Bottom) => State::Bottom,
                _ => new_state.try_put(a1, lhs).try_put(a2, rhs),
            }
        }

        BooleanExpr::NumLtEq(a1, a2) => {
            let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
            let lhs = i1.intersection(i2.add_min(Integer::NegInf));
            let rhs = i2.intersection(i1.add_max(Integer::PosInf));
            match (lhs, rhs) {
                (Interval::Bottom, _) | (_, Interval::Bottom) => State::Bottom,
                _ => new_state.try_put(a1, lhs).try_put(a2, rhs),
            }
        }

        BooleanExpr::NumGt(a1, a2) => {
            eval_bexpr(&BooleanExpr::NumLt(a2.clone(), a1.clone()), state)
        }
        BooleanExpr::NumGtEq(a1, a2) => {
            eval_bexpr(&BooleanExpr::NumLtEq(a2.clone(), a1.clone()), state)
        }
    }
}

// --- helpers

fn trans_aexpr(
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, Interval, State) {
    let (i1, new_state) = eval_aexpr(a1, &state);
    let (i2, new_state) = eval_aexpr(a2, &new_state);
    (i1, i2, new_state)
}

fn binop_aexpr(
    op: fn(Interval, Interval) -> Interval,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, State) {
    let (i1, i2, new_state) = trans_aexpr(a1, a2, &state);
    (op(i1, i2), new_state)
}
