use crate::parser::ast::*;
use std::fmt::{self};

pub enum ProgramPoint {
    Skip(Statement),
    Assignment(Statement),

    IfGuard(BooleanExpr),
    ElseGuard(BooleanExpr),
    EndIf,

    WhileInv { delay: i64 },
    WhileGuard(BooleanExpr),
    EndWhile(BooleanExpr),
}

impl fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPoint::Skip(s) => write!(f, "{}", s),
            ProgramPoint::Assignment(s) => write!(f, "{}", s),

            ProgramPoint::IfGuard(b) => write!(f, "[if-guard] {}", b),
            ProgramPoint::ElseGuard(b) => write!(f, "[else-guard] {}", b),
            ProgramPoint::EndIf => write!(f, "[end-if]"),

            ProgramPoint::WhileInv { delay } => write!(f, "[while-inv] @delay:{}", delay),
            ProgramPoint::WhileGuard(b) => write!(f, "[while-guard] {}", b),
            ProgramPoint::EndWhile(b) => write!(f, "[end-while] {}", b),
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
            vec![ProgramPoint::WhileInv {
                delay: get_loop_delay(&stmt),
            }],
            vec![ProgramPoint::WhileGuard(*cond.clone())],
            get_program_points(*body),
            vec![ProgramPoint::EndWhile(cond.negate())],
        ]),

        RepeatUntil { body, cond, .. } => concat(vec![
            get_program_points(*body.clone()),
            vec![ProgramPoint::WhileInv {
                delay: get_loop_delay(&stmt),
            }],
            vec![ProgramPoint::WhileGuard(*cond.clone())],
            get_program_points(*body),
            vec![ProgramPoint::EndWhile(cond.negate())],
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
