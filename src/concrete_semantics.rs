use crate::ast::*;
use crate::concrete_state::*;

// --- type aliases

type StatePredicate = Box<dyn Fn(State) -> (bool, State)>;
type StateFunction = Box<dyn Fn(State) -> Option<State>>;
type Functional = Box<dyn Fn(StateFunction) -> StateFunction>;

// --- semantic functions

fn bottom() -> StateFunction {
    Box::new(|_| None)
}

fn id() -> StateFunction {
    Box::new(|state| Some(state))
}

fn compose(f: StateFunction, g: StateFunction) -> StateFunction {
    Box::new(move |state| match f(state.clone()) {
        Some(new_state) => g(new_state),
        None => None,
    })
}

fn state_update(var: String, val: ArithmeticExpr) -> StateFunction {
    Box::new(move |state| {
        let (val, new_state) = denote_aexpr(&val, &state);
        let new_state = write(new_state, &var, val);
        Some(new_state)
    })
}

fn predicate(cond: BooleanExpr) -> StatePredicate {
    Box::new(move |state| denote_bexpr(&cond, &state))
}

fn conditional(p: StatePredicate, s1: StateFunction, s2: StateFunction) -> StateFunction {
    Box::new(move |state| match p(state.clone()) {
        (true, new_state) => s1(new_state),
        (false, new_state) => s2(new_state),
    })
}

fn fix(f: Functional) -> StateFunction {
    Box::new(move |state| {
        let mut g = bottom();
        loop {
            g = f(g);
            let final_state = g(state.clone());
            if final_state.is_some() {
                return final_state;
            }
        }
    })
}

// --- ast denotation

pub fn denote_statement(ast: Statement) -> StateFunction {
    match ast {
        Statement::Skip => id(),
        Statement::Chain(s1, s2) => compose(denote_statement(*s1), denote_statement(*s2)),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::If { cond, s1, s2 } => conditional(
            predicate(*cond),
            denote_statement(*s1),
            denote_statement(*s2),
        ),
        Statement::While { cond, body } => {
            let f = Box::new(move |g| {
                conditional(
                    predicate(*cond.clone()),
                    compose(denote_statement(*body.clone()), g),
                    id(),
                )
            });
            fix(f)
        }
    }
}

pub fn denote_aexpr(expr: &ArithmeticExpr, state: &State) -> (i32, State) {
    match expr {
        ArithmeticExpr::Number(n) => (*n, state.clone()),
        ArithmeticExpr::Identifier(var) => (read(state, var), state.clone()),
        ArithmeticExpr::PostIncrement(var) => {
            let val = read(state, var);
            (val, write(state.clone(), var, val + 1))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let val = read(state, var);
            (val, write(state.clone(), var, val - 1))
        }
        ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
        ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
        ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
        ArithmeticExpr::Div(a1, a2) => binop_aexpr(|a, b| a / b, a1, a2, state),
    }
}

pub fn denote_bexpr(expr: &BooleanExpr, state: &State) -> (bool, State) {
    match expr {
        BooleanExpr::True => (true, state.clone()),
        BooleanExpr::False => (false, state.clone()),
        BooleanExpr::Not(b) => unop_bexpr(|b| !b, b, state),
        BooleanExpr::And(b1, b2) => binop_bexpr(|a, b| a && b, b1, b2, state),
        BooleanExpr::NumEq(a1, a2) => binop_comp(|a, b| a == b, a1, a2, state),
        BooleanExpr::NumLt(a1, a2) => binop_comp(|a, b| a < b, a1, a2, state),
        BooleanExpr::NumGt(a1, a2) => binop_comp(|a, b| a > b, a1, a2, state),
        BooleanExpr::NumLtEq(a1, a2) => binop_comp(|a, b| a <= b, a1, a2, state),
        BooleanExpr::NumGtEq(a1, a2) => binop_comp(|a, b| a >= b, a1, a2, state),
    }
}

// --- helpers

fn binop_aexpr(
    op: fn(i32, i32) -> i32,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (i32, State) {
    let (val1, state1) = denote_aexpr(a1, &state);
    let (val2, state2) = denote_aexpr(a2, &state1);
    (op(val1, val2), state2)
}

fn binop_bexpr(
    op: fn(bool, bool) -> bool,
    b1: &BooleanExpr,
    b2: &BooleanExpr,
    state: &State,
) -> (bool, State) {
    let (val1, state1) = denote_bexpr(b1, &state);
    let (val2, state2) = denote_bexpr(b2, &state1);
    (op(val1, val2), state2)
}

fn unop_bexpr(op: fn(bool) -> bool, b: &BooleanExpr, state: &State) -> (bool, State) {
    let (val, state) = denote_bexpr(b, &state);
    (op(val), state)
}

fn binop_comp(
    op: fn(i32, i32) -> bool,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (bool, State) {
    let (val1, state1) = denote_aexpr(a1, &state);
    let (val2, state2) = denote_aexpr(a2, &state1);
    (op(val1, val2), state2)
}

/*
--- ALTERNATIVE IMPLEMENTATIONS

fn rec_self_apply(f: &Functional, n: i32, inp: StateFunction) -> StateFunction {
    match n {
        0 => inp,
        _ => rec_self_apply(f, n - 1, f(inp)),
    }
}

fn fix3(f: Functional) -> StateFunction {
    Box::new(move |state| {
        let mut n = 0;
        loop {
            let new_state = rec_self_apply(&f, n, bottom())(state.clone());
            if new_state.is_some() {
                return new_state;
            }
            n += 1;
        }
    })
}

fn fix2(f: Functional) -> StateFunction {
    Box::new(move |state| {
        let mut n = 0;

        loop {
            let mut g = bottom();
            for _ in 0..n {
                g = f(g);
            }

            let final_state = g(state.clone());
            if final_state.is_some() {
                return final_state;
            }

            n += 1;
        }
    })
}
*/
