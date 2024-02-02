use crate::abstract_state::*;
use crate::ast::*;
use crate::interval_domain::*;

// --- type aliases

type StateFunction = Box<dyn Fn(State, ProgramPoints) -> (State, ProgramPoints)>;

fn id() -> StateFunction {
    Box::new(|state, points| (state.clone(), concat(&[points, vec![state]])))
}

fn compose(f: StateFunction, g: StateFunction) -> StateFunction {
    Box::new(move |state, points| {
        let (f_state, f_points) = f(state, points);
        g(f_state, f_points)
    })
}

fn state_update(var: String, val: ArithmeticExpr) -> StateFunction {
    Box::new(move |state, points| {
        let (val, new_state) = denote_aexpr(&val, &state);
        let new_state = new_state.update(&var, val);
        (new_state.clone(), concat(&[points, vec![new_state]]))
    })
}

fn conditional(cond: BooleanExpr, s1: StateFunction, s2: StateFunction) -> StateFunction {
    Box::new(move |state, points| {
        let tt_state = denote_bexpr(&cond, &state);
        let ff_state = denote_bexpr(&BooleanExpr::Not(Box::new(cond.clone())), &state);

        let (s1_state, s1_points) = s1(tt_state.clone(), vec![]);
        let (s2_state, s2_points) = s2(ff_state.clone(), vec![]);

        (
            s1_state.union(&s2_state),
            concat(&[points, vec![tt_state], s1_points, vec![ff_state], s2_points]),
        )
    })
}

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
        ArithmeticExpr::Number(n) => (interval_from_value(*n), state.clone()),
        ArithmeticExpr::Identifier(var) => (state.read(var), state.clone()),
        ArithmeticExpr::PostIncrement(var) => {
            let val = state.read(var);
            (val, state.update(var, interval_add_value(val, 1)))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let val = state.read(var);
            (val, state.update(var, interval_add_value(val, -1)))
        }
        ArithmeticExpr::Add(a1, a2) => binop_aexpr(todo!(), a1, a2, state),
        ArithmeticExpr::Sub(a1, a2) => binop_aexpr(todo!(), a1, a2, state),
        ArithmeticExpr::Mul(a1, a2) => binop_aexpr(todo!(), a1, a2, state),
        ArithmeticExpr::Div(a1, a2) => binop_aexpr(todo!(), a1, a2, state),
    }
}

pub fn denote_bexpr(expr: &BooleanExpr, state: &State) -> State {
    match expr {
        BooleanExpr::True => state.clone(),
        BooleanExpr::False => None,
        BooleanExpr::Not(b) => denote_bexpr(&desugar_not_bexpr(*b.clone()), state),
        BooleanExpr::And(b1, b2) => denote_bexpr(b1, state).intersect(&denote_bexpr(b1, state)),
        BooleanExpr::Or(b1, b2) => denote_bexpr(b1, state).union(&denote_bexpr(b1, state)),
        BooleanExpr::NumEq(a1, a2) => {
            let (a1_val, a1_state) = denote_aexpr(a1, state);
            let (a2_val, a2_state) = denote_aexpr(a2, &a1_state);

            match (a1_val, a2_val) {
                (Interval::Bottom, _) | (_, Interval::Bottom) => a2_state,
                _ => match interval_contains(a1_val, a2_val) {
                    false => create_empty(),
                    true => a2_state,
                },
            }
        }
        BooleanExpr::NumNotEq(a1, a2) => todo!(),
        BooleanExpr::NumLt(a1, a2) => todo!(),
        BooleanExpr::NumGt(a1, a2) => todo!(),
        BooleanExpr::NumLtEq(a1, a2) => todo!(),
        BooleanExpr::NumGtEq(a1, a2) => todo!(),
    }
}

// --- helpers

fn binop_aexpr(
    op: fn(Interval, Interval) -> Interval,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Interval, State) {
    let (val1, state1) = denote_aexpr(a1, &state);
    let (val2, state2) = denote_aexpr(a2, &state1);
    (op(val1, val2), state2)
}
