use crate::parser::ast::*;
use std::fmt::{self};

pub enum ProgramPoint {
    Stmt(Statement),

    IfGuard(BooleanExpr),
    ElseGuard(BooleanExpr),
    EndIf,

    WhileGuard(BooleanExpr),
    EndWhile(BooleanExpr),

    Repeat,
    UntilGuard(BooleanExpr),
    EndRepeatUntil(BooleanExpr),
}

impl fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPoint::Stmt(s) => write!(f, "{}", s),
            ProgramPoint::IfGuard(b) => write!(f, "[if-guard] {}", b),
            ProgramPoint::ElseGuard(b) => write!(f, "[else-guard] {}", b),
            ProgramPoint::EndIf => write!(f, "[end-if]"),
            ProgramPoint::WhileGuard(b) => write!(f, "[while-guard] {}", b),
            ProgramPoint::EndWhile(b) => write!(f, "[end-while] {}", b),
            ProgramPoint::Repeat => write!(f, "[repeat]"),
            ProgramPoint::UntilGuard(b) => write!(f, "[until-guard] {}", b),
            ProgramPoint::EndRepeatUntil(b) => write!(f, "[end-repeat] {}", b),
        }
    }
}

pub fn get_program_points(stmt: Statement) -> Vec<ProgramPoint> {
    use ProgramPoint::*;
    use Statement::*;

    match stmt.clone() {
        Skip => vec![Stmt(stmt)],
        Assignment { var: _, val: _ } => vec![Stmt(stmt)],
        Chain(s1, s2) => concat(vec![get_program_points(*s1), get_program_points(*s2)]),

        If { cond, s1, s2 } => concat(vec![
            vec![IfGuard(*cond.clone())],
            get_program_points(*s1),
            vec![ElseGuard(negate_bexpr(&cond))],
            get_program_points(*s2),
            vec![EndIf],
        ]),

        While { cond, body } => concat(vec![
            vec![WhileGuard(*cond.clone())],
            get_program_points(*body),
            vec![EndWhile(negate_bexpr(&cond))],
        ]),

        RepeatUntil { body, cond } => concat(vec![
            vec![Repeat],
            get_program_points(*body),
            vec![UntilGuard(negate_bexpr(&cond))],
            vec![EndRepeatUntil(*cond.clone())],
        ]),
    }
}

fn concat(vs: Vec<Vec<ProgramPoint>>) -> Vec<ProgramPoint> {
    let mut new: Vec<ProgramPoint> = vec![];
    for v in vs {
        new.extend(v);
    }
    new
}
