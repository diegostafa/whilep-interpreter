use std::fmt::{self};

use crate::parser::ast::*;

pub enum ProgramPoint {
    Stmt(Statement),
    IfGuard(BooleanExpr),
    ElseGuard(BooleanExpr),
    EndIf,
    WhileGuard(BooleanExpr),
    EndWhile,
}

impl fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramPoint::Stmt(s) => match s {
                Statement::Skip | Statement::Assignment { var: _, val: _ } => {
                    write!(f, "{}", s)
                }
                _ => unreachable!(),
            },
            ProgramPoint::IfGuard(b) => write!(f, "[if-guard] {}", b),
            ProgramPoint::ElseGuard(b) => write!(f, "[else-guard] {}", b),
            ProgramPoint::EndIf => write!(f, "[end-if]"),
            ProgramPoint::WhileGuard(b) => write!(f, "[while-guard] {}", b),
            ProgramPoint::EndWhile => write!(f, "[end-while]"),
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
            vec![WhileGuard(*cond)],
            get_program_points(*body),
            vec![EndWhile],
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
