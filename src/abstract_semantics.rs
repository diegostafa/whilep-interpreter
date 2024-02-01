use crate::abstract_domain::*;
use crate::abstract_state::*;
use crate::ast::*;

// --- type aliases

type StateFunction = Box<dyn Fn(State, ProgramPoints) -> (State, ProgramPoints)>;
type Functional = Box<dyn Fn(StateFunction) -> StateFunction>;

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
        let new_state = write(new_state, &var, val);
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
            union(&s1_state, &s2_state),
            concat(&[points, vec![tt_state], s1_points, vec![ff_state], s2_points]),
        )
    })
}

pub fn denote_statement(ast: Statement) -> StateFunction {
    match ast {
        Statement::Skip => id(),
        Statement::Assignment { var, val } => state_update(var, *val),
        Statement::Chain(s1, s2) => compose(denote_statement(*s1), denote_statement(*s2)),

        Statement::If { cond, s1, s2 } => {
            conditional(*cond, denote_statement(*s1), denote_statement(*s2))
        }

        Statement::While { cond, body } => todo!(),
    }
}

pub fn denote_aexpr(expr: &ArithmeticExpr, state: &State) -> (Interval, State) {
    match expr {
        ArithmeticExpr::Number(n) => (Interval::from_value(*n), state.clone()),
        ArithmeticExpr::Identifier(var) => (read(state, var), state.clone()),
        ArithmeticExpr::PostIncrement(var) => {
            let val = read(state, var);
            (val, write(state.clone(), var, val.add_value(1)))
        }
        ArithmeticExpr::PostDecrement(var) => {
            let val = read(state, var);
            (val, write(state.clone(), var, val.add_value(-1)))
        }
        ArithmeticExpr::Add(a1, a2) => todo!(),
        ArithmeticExpr::Sub(a1, a2) => todo!(),
        ArithmeticExpr::Mul(a1, a2) => todo!(),
        ArithmeticExpr::Div(a1, a2) => todo!(),
    }
}

pub fn denote_bexpr(expr: &BooleanExpr, state: &State) -> State {
    match expr {
        BooleanExpr::True => todo!(),
        BooleanExpr::False => todo!(),
        BooleanExpr::Not(b) => todo!(),
        BooleanExpr::And(b1, b2) => todo!(),
        BooleanExpr::NumEq(a1, a2) => todo!(),
        BooleanExpr::NumLt(a1, a2) => todo!(),
        BooleanExpr::NumGt(a1, a2) => todo!(),
        BooleanExpr::NumLtEq(a1, a2) => todo!(),
        BooleanExpr::NumGtEq(a1, a2) => todo!(),
    }
}
