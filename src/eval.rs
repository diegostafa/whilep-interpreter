use crate::ast::*;
use crate::eval;
use crate::state::*;

type StateFunction = Box<dyn Fn(State) -> State>;

fn id() -> StateFunction {
    Box::new(|state| state)
}

fn compose(f: StateFunction, g: StateFunction) -> StateFunction {
    Box::new(move |state| g(f(state)))
}

fn FIX(F: fn(StateFunction) -> StateFunction) -> StateFunction {
    Box::new(|state| todo!())
}

pub fn make_denotational(ast: StatementExpr) -> StateFunction {
    match ast {
        StatementExpr::Skip => id(),

        StatementExpr::Chain(s1, s2) => {
            let f = make_denotational(*s1);
            let g = make_denotational(*s2);
            compose(f, g)
        }

        StatementExpr::Assignment { var, val } => Box::new(move |state| {
            let val = eval::arithmetic_expr(&val, &state);
            let mut new_state = state.clone();
            new_state.insert(var.to_string(), val);
            new_state
        }),

        StatementExpr::If { cond, s1, s2 } => {
            let d1 = make_denotational(*s1);
            let d2 = make_denotational(*s2);

            Box::new(move |state| match eval::boolean_expr(&cond, &state) {
                true => d1(state),
                false => d2(state),
            })
        }

        StatementExpr::While { cond, body } => {
            todo!()
        }
    }
}

pub fn arithmetic_expr(expr: &ArithmeticExpr, state: &State) -> i32 {
    match expr {
        ArithmeticExpr::Number(n) => *n,
        ArithmeticExpr::Identifier(var) => state.get(var).unwrap().to_owned(),
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
