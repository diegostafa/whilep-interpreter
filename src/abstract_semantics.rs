use crate::abstract_state::*;
use crate::ast::*;
use crate::interval::*;
use crate::lattice::*;

// --- type aliases

type StateFunction = Box<dyn Fn((State, Invariant)) -> (State, Invariant)>;

// --- semantic functions

fn id() -> StateFunction {
    Box::new(|(state, inv)| (state.clone(), concat(&[inv, vec![state]])))
}

fn compose(f: StateFunction, g: StateFunction) -> StateFunction {
    Box::new(move |(state, inv)| g(f((state, inv))))
}

fn state_update(var: String, val: ArithmeticExpr) -> StateFunction {
    Box::new(move |(state, inv)| {
        let (val, new_state) = denote_aexpr(&val, &state);
        let new_state = new_state.put(&var, val);
        (new_state.clone(), concat(&[inv, vec![new_state]]))
    })
}

fn conditional(cond: BooleanExpr, s1: StateFunction, s2: StateFunction) -> StateFunction {
    Box::new(move |(state, inv)| {
        let tt_state = denote_bexpr(&cond, &state);
        let ff_state = denote_bexpr(&BooleanExpr::Not(Box::new(cond.clone())), &state);
        let (s1_state, s1_inv) = s1((tt_state.clone(), default_invariant()));
        let (s2_state, s2_inv) = s2((ff_state.clone(), default_invariant()));

        (
            s1_state.union(s2_state),
            concat(&[inv, vec![tt_state], s1_inv, vec![ff_state], s2_inv]),
        )
    })
}

fn fix(f: StateFunction) -> StateFunction {
    Box::new(move |(state, inv)| todo!())
}

// --- ast denotation

pub fn denote_stmt(ast: Statement) -> StateFunction {
    match ast {
        Statement::Skip => id(),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),
        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),
        Statement::While { cond, body } => todo!(),
    }
}

pub fn denote_aexpr(expr: &ArithmeticExpr, state: &State) -> (Interval, State) {
    match expr {
        ArithmeticExpr::Number(n) => (Interval::Range(*n, *n), state.clone()),
        ArithmeticExpr::Identifier(var) => (state.read(var), state.clone()),
        ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
        ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
        ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
        ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),

        ArithmeticExpr::PostIncrement(var) => {
            let interval = state.read(var);
            (interval, state.put(var, interval.add_i32(1)))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let interval = state.read(var);
            (interval, state.put(var, interval.add_i32(-1)))
        }
        ArithmeticExpr::Interval(a1, a2) => {
            let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
            match (a1_interval, a2_interval) {
                (Interval::Range(a, _), Interval::Range(_, d)) if a <= d => {
                    (Interval::Range(a, d), new_state)
                }
                _ => (Interval::Bottom, state.clone()),
            }
        }
    }
}

pub fn denote_bexpr(expr: &BooleanExpr, state: &State) -> State {
    match expr {
        BooleanExpr::True => state.clone(),
        BooleanExpr::False => None,
        BooleanExpr::Not(b) => denote_bexpr(&desugar_not_bexpr(*b.clone()), state),
        BooleanExpr::And(b1, b2) => denote_bexpr(b1, state).intersection(denote_bexpr(b2, state)),
        BooleanExpr::Or(b1, b2) => denote_bexpr(b1, state).union(denote_bexpr(b2, state)),
        BooleanExpr::NumEq(a1, a2) => binop_cmp(|i1, i2| i1.overlaps(i2), a1, a2, state),
        BooleanExpr::NumNotEq(a1, a2) => binop_cmp(|i1, i2| !i1.overlaps(i2), a1, a2, state),
        BooleanExpr::NumLt(a1, a2) => binop_cmp(|i1, i2| !i1.on_right(i2), a1, a2, state),
        BooleanExpr::NumGt(a1, a2) => binop_cmp(|i1, i2| !i1.on_left(i2), a1, a2, state),
        BooleanExpr::NumLtEq(a1, a2) => {
            binop_cmp(|i1, i2| !i1.on_right(i2) && !i1.overlaps(i2), a1, a2, state)
        }
        BooleanExpr::NumGtEq(a1, a2) => {
            binop_cmp(|i1, i2| !i1.on_left(i2) && !i1.overlaps(i2), a1, a2, state)
        }
    }
}

// --- helpers

fn trans_aexpr(
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, Interval, State) {
    let (a1_interval, new_state) = denote_aexpr(a1, &state);
    let (a2_interval, new_state) = denote_aexpr(a2, &new_state);
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

fn binop_cmp(
    cond: fn(Interval, Interval) -> bool,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> State {
    let (a1_interval, a2_interval, new_state) = trans_aexpr(a1, a2, &state);
    match cond(a1_interval, a2_interval) {
        true => new_state,
        _ => None,
    }
}
