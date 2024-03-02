use crate::concrete_semantics::state::*;
use crate::parser::ast::*;
use crate::types::integer::*;

// --- type aliases

type StateFunction = Box<dyn Fn(State) -> Option<State>>;
type Functional = Box<dyn Fn(StateFunction) -> StateFunction>;

trait FunctionMethods {
    fn compose_many(&self, n: i32, input: StateFunction) -> StateFunction;
}

impl FunctionMethods for Functional {
    fn compose_many(&self, n: i32, input: StateFunction) -> StateFunction {
        match n {
            0 => input,
            _ => self.compose_many(n - 1, self(input)),
        }
    }
}

// --- ast denotation

pub fn denote_stmt(stmt: Statement) -> StateFunction {
    match stmt {
        Statement::Skip => id(),
        Statement::Chain(s1, s2) => compose(denote_stmt(*s1), denote_stmt(*s2)),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::If { cond, s1, s2 } => conditional(*cond, denote_stmt(*s1), denote_stmt(*s2)),
        Statement::While { cond, body } => {
            let f = Box::new(move |g| {
                conditional(*cond.clone(), compose(denote_stmt(*body.clone()), g), id())
            });
            fix(f)
        }
        Statement::RepeatUntil { body, cond } => {
            let f = Box::new(move |g| {
                compose(
                    denote_stmt(*body.clone()),
                    conditional(*cond.clone(), id(), g),
                )
            });
            fix(f)
        }
    }
}

pub fn eval_aexpr(expr: &ArithmeticExpr, state: &State) -> (Integer, State) {
    match expr {
        ArithmeticExpr::Number(n) => (*n, state.clone()),
        ArithmeticExpr::Interval(n, m) => (random_integer_between(*n, *m), state.clone()),
        ArithmeticExpr::Variable(var) => (state.read(var), state.clone()),
        ArithmeticExpr::Add(a1, a2) => binop_aexpr(|a, b| a + b, a1, a2, state),
        ArithmeticExpr::Sub(a1, a2) => binop_aexpr(|a, b| a - b, a1, a2, state),
        ArithmeticExpr::Mul(a1, a2) => binop_aexpr(|a, b| a * b, a1, a2, state),
        ArithmeticExpr::Div(a1, a2) => {
            let (a1_val, new_state) = eval_aexpr(a1, &state);
            let (a2_val, new_state) = eval_aexpr(a2, &new_state);
            match a2_val {
                Integer::Value(0) => panic!("[ERROR] division by zero"),
                _ => (a1_val / a2_val, new_state),
            }
        }
        ArithmeticExpr::PostIncrement(var) => {
            let val = state.read(var);
            (val, state.put(var, val + 1))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let val = state.read(var);
            (val, state.put(var, val - 1))
        }
    }
}

pub fn eval_bexpr(expr: &BooleanExpr, state: &State) -> (bool, State) {
    match expr {
        BooleanExpr::True => (true, state.clone()),
        BooleanExpr::False => (false, state.clone()),
        BooleanExpr::Not(b) => eval_bexpr(&negate_bexpr(b), state),
        BooleanExpr::And(b1, b2) => binop_bexpr(|a, b| a && b, b1, b2, state),
        BooleanExpr::Or(b1, b2) => binop_bexpr(|a, b| a || b, b1, b2, state),
        BooleanExpr::NumEq(a1, a2) => binop_cmp(|a, b| a == b, a1, a2, state),
        BooleanExpr::NumNotEq(a1, a2) => binop_cmp(|a, b| a != b, a1, a2, state),
        BooleanExpr::NumLt(a1, a2) => binop_cmp(|a, b| a < b, a1, a2, state),
        BooleanExpr::NumGt(a1, a2) => binop_cmp(|a, b| a > b, a1, a2, state),
        BooleanExpr::NumLtEq(a1, a2) => binop_cmp(|a, b| a <= b, a1, a2, state),
        BooleanExpr::NumGtEq(a1, a2) => binop_cmp(|a, b| a >= b, a1, a2, state),
    }
}

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

fn state_update(var: Identifier, val: ArithmeticExpr) -> StateFunction {
    Box::new(move |state| {
        let (val, new_state) = eval_aexpr(&val, &state);
        Some(new_state.put(&var, val))
    })
}

fn conditional(cond: BooleanExpr, s1: StateFunction, s2: StateFunction) -> StateFunction {
    Box::new(move |state| match eval_bexpr(&cond, &state) {
        (true, new_state) => s1(new_state),
        (_, new_state) => s2(new_state),
    })
}

fn fix(f: Functional) -> StateFunction {
    Box::new(move |state| {
        let mut g = bottom();
        loop {
            g = f(g);
            if let Some(state) = g(state.clone()) {
                return Some(state);
            }
        }
    })
}

// alternative implementation
fn _fix(f: Functional) -> StateFunction {
    Box::new(move |state| {
        let mut n = 0;
        loop {
            n += 1;
            let g = f.compose_many(n, bottom());
            if let Some(state) = g(state.clone()) {
                return Some(state);
            }
        }
    })
}

// --- helpers

fn binop_aexpr(
    op: fn(Integer, Integer) -> Integer,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (Integer, State) {
    let (a1_val, new_state) = eval_aexpr(a1, &state);
    let (a2_val, new_state) = eval_aexpr(a2, &new_state);
    (op(a1_val, a2_val), new_state)
}

fn binop_bexpr(
    op: fn(bool, bool) -> bool,
    b1: &BooleanExpr,
    b2: &BooleanExpr,
    state: &State,
) -> (bool, State) {
    let (b1_val, new_state) = eval_bexpr(b1, &state);
    let (b2_val, new_state) = eval_bexpr(b2, &new_state);
    (op(b1_val, b2_val), new_state)
}

fn binop_cmp(
    op: fn(Integer, Integer) -> bool,
    a1: &ArithmeticExpr,
    a2: &ArithmeticExpr,
    state: &State,
) -> (bool, State) {
    let (a1_val, new_state) = eval_aexpr(a1, &state);
    let (a2_val, new_state) = eval_aexpr(a2, &new_state);
    (op(a1_val, a2_val), new_state)
}
