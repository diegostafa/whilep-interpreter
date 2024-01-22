use crate::ast::*;
use crate::eval;
use crate::state::*;

// --- type aliases

type StatePredicate = Box<dyn Fn(State) -> bool>;
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
        let mut new_state = state.clone();
        new_state.insert(var.clone(), eval::arithmetic_expr(&val, &state));
        Some(new_state)
    })
}

fn predicate(cond: BooleanExpr) -> StatePredicate {
    Box::new(move |state| eval::boolean_expr(&cond, &state))
}

fn conditional(p: StatePredicate, tt: StateFunction, ff: StateFunction) -> StateFunction {
    Box::new(move |state| match p(state.clone()) {
        true => tt(state),
        false => ff(state),
    })
}

fn rec_self_apply(f: &Functional, n: i32, input: StateFunction) -> StateFunction {
    match n {
        0 => input,
        _ => rec_self_apply(f, n - 1, f(input)),
    }
}

fn fix(f: Functional) -> StateFunction {
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

// --- evaluators

pub fn induced_function(ast: StatementExpr) -> StateFunction {
    match ast {
        StatementExpr::Skip => id(),
        StatementExpr::Chain(s1, s2) => compose(induced_function(*s1), induced_function(*s2)),
        StatementExpr::Assignment { var, val } => state_update(var, *val),
        StatementExpr::If { cond, s1, s2 } => conditional(
            predicate(*cond),
            induced_function(*s1),
            induced_function(*s2),
        ),
        StatementExpr::While { cond, body } => {
            let induced_functional = Box::new(move |g| {
                conditional(
                    predicate(*cond.clone()),
                    compose(induced_function(*body.clone()), g),
                    id(),
                )
            });
            fix(induced_functional)
        }
    }
}

pub fn arithmetic_expr(expr: &ArithmeticExpr, state: &State) -> i32 {
    match expr {
        ArithmeticExpr::Number(n) => *n,
        ArithmeticExpr::Identifier(var) => *state.get(var).unwrap(),
        ArithmeticExpr::Add(a1, a2) => arithmetic_expr(a1, state) + arithmetic_expr(a2, state),
        ArithmeticExpr::Sub(a1, a2) => arithmetic_expr(a1, state) - arithmetic_expr(a2, state),
        ArithmeticExpr::Mul(a1, a2) => arithmetic_expr(a1, state) * arithmetic_expr(a2, state),
        ArithmeticExpr::Div(a1, a2) => arithmetic_expr(a1, state) / arithmetic_expr(a2, state),
    }
}

pub fn boolean_expr(expr: &BooleanExpr, state: &State) -> bool {
    match expr {
        BooleanExpr::True => true,
        BooleanExpr::False => false,
        BooleanExpr::Not(b) => !boolean_expr(b, state),
        BooleanExpr::And(b1, b2) => boolean_expr(b1, state) && boolean_expr(b2, state),
        BooleanExpr::NumEq(a1, a2) => arithmetic_expr(a1, state) == arithmetic_expr(a2, state),
        BooleanExpr::NumLtEq(a1, a2) => arithmetic_expr(a1, state) <= arithmetic_expr(a2, state),
    }
}
