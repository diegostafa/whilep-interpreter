use crate::parser::ast::*;
use std::fmt::{self};

pub enum ProgramPoint {
    Skip(Statement),
    Assignment(Statement),

    IfGuard(BooleanExpr),
    ElseGuard(BooleanExpr),
    EndIf,

    WhileInv,
    WhileGuard(BooleanExpr),
    EndWhile(BooleanExpr),

    Repeat,
    UntilGuard(BooleanExpr),
    EndRepeatUntil(BooleanExpr),
}

impl fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPoint::Skip(s) => write!(f, "{}", s),
            ProgramPoint::Assignment(s) => write!(f, "{}", s),

            ProgramPoint::IfGuard(b) => write!(f, "[if-guard] {}", b),
            ProgramPoint::ElseGuard(b) => write!(f, "[else-guard] {}", b),
            ProgramPoint::EndIf => write!(f, "[end-if]"),

            ProgramPoint::WhileInv => write!(f, "[while-inv]"),
            ProgramPoint::WhileGuard(b) => write!(f, "[while-guard] {}", b),
            ProgramPoint::EndWhile(b) => write!(f, "[end-while] {}", b),

            ProgramPoint::Repeat => write!(f, "[repeat]"),
            ProgramPoint::UntilGuard(b) => write!(f, "[until-guard] {}", b),
            ProgramPoint::EndRepeatUntil(b) => write!(f, "[end-repeat] {}", b),
        }
    }
}

pub fn get_program_points(stmt: Statement) -> Vec<ProgramPoint> {
    use Statement::*;

    match stmt.clone() {
        Skip => vec![ProgramPoint::Skip(stmt)],
        Assignment { var: _, val: _ } => vec![ProgramPoint::Assignment(stmt)],
        Chain(s1, s2) => concat(vec![get_program_points(*s1), get_program_points(*s2)]),

        If { cond, s1, s2 } => concat(vec![
            vec![ProgramPoint::IfGuard(*cond.clone())],
            get_program_points(*s1),
            vec![ProgramPoint::ElseGuard(cond.negate())],
            get_program_points(*s2),
            vec![ProgramPoint::EndIf],
        ]),

        While { cond, body, .. } => concat(vec![
            vec![ProgramPoint::WhileInv],
            vec![ProgramPoint::WhileGuard(*cond.clone())],
            get_program_points(*body),
            vec![ProgramPoint::EndWhile(cond.negate())],
        ]),

        RepeatUntil { body, cond, .. } => concat(vec![
            vec![ProgramPoint::Repeat],
            get_program_points(*body),
            vec![ProgramPoint::UntilGuard(cond.negate())],
            vec![ProgramPoint::EndRepeatUntil(*cond.clone())],
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
