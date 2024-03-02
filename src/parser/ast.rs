use crate::types::integer::*;
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};
use std::fmt;

lalrpop_mod!(pub whilep);

pub type Identifier = String;

pub fn parse(source: &str) -> Result<Statement, ParseError<usize, Token, &'static str>> {
    whilep::StmtParser::new().parse(source)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Skip,
    Chain(Box<Statement>, Box<Statement>),
    Assignment {
        var: Identifier,
        val: Box<ArithmeticExpr>,
    },
    If {
        cond: Box<BooleanExpr>,
        s1: Box<Statement>,
        s2: Box<Statement>,
    },
    While {
        cond: Box<BooleanExpr>,
        body: Box<Statement>,
    },

    RepeatUntil {
        body: Box<Statement>,
        cond: Box<BooleanExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum ArithmeticExpr {
    Number(Integer),
    Interval(Integer, Integer),
    Variable(Identifier),
    Add(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Sub(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Mul(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Div(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    PostIncrement(Identifier),
    PostDecrement(Identifier),
}

#[derive(Debug, Clone)]
pub enum BooleanExpr {
    True,
    False,
    Not(Box<BooleanExpr>),
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    NumEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumNotEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumLt(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumGt(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumLtEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumGtEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
}

pub fn negate_bexpr(expr: &BooleanExpr) -> BooleanExpr {
    match expr.clone() {
        BooleanExpr::True => BooleanExpr::False,
        BooleanExpr::False => BooleanExpr::True,
        BooleanExpr::Not(b) => *b,
        BooleanExpr::And(b1, b2) => BooleanExpr::Or(
            Box::new(BooleanExpr::Not(b1)),
            Box::new(BooleanExpr::Not(b2)),
        ),
        BooleanExpr::Or(b1, b2) => BooleanExpr::And(
            Box::new(BooleanExpr::Not(b1)),
            Box::new(BooleanExpr::Not(b2)),
        ),
        BooleanExpr::NumEq(a1, a2) => BooleanExpr::NumNotEq(a1, a2),
        BooleanExpr::NumNotEq(a1, a2) => BooleanExpr::NumEq(a1, a2),
        BooleanExpr::NumLt(a1, a2) => BooleanExpr::NumGtEq(a1, a2),
        BooleanExpr::NumGt(a1, a2) => BooleanExpr::NumLtEq(a1, a2),
        BooleanExpr::NumLtEq(a1, a2) => BooleanExpr::NumGt(a1, a2),
        BooleanExpr::NumGtEq(a1, a2) => BooleanExpr::NumLt(a1, a2),
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Skip => write!(f, "skip"),
            Statement::Assignment { var, val } => write!(f, "{} := {}", var, val),
            Statement::Chain(s1, s2) => write!(f, "{}; {}", s1, s2),
            Statement::If { cond, s1, s2 } => write!(f, "if {} then {} else {} end", cond, s1, s2),
            Statement::While { cond, body } => write!(f, "while {} do {} done", cond, body),
            Statement::RepeatUntil { cond, body } => write!(f, "repeat {} until {}", body, cond),
        }
    }
}

impl fmt::Display for ArithmeticExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticExpr::Number(n) => write!(f, "{}", n),
            ArithmeticExpr::Interval(a, b) => write!(f, "[{}, {}]", a, b),
            ArithmeticExpr::Variable(s) => write!(f, "{}", s),
            ArithmeticExpr::Add(a, b) => write!(f, "({} + {})", a, b),
            ArithmeticExpr::Sub(a, b) => write!(f, "({} - {})", a, b),
            ArithmeticExpr::Mul(a, b) => write!(f, "({} * {})", a, b),
            ArithmeticExpr::Div(a, b) => write!(f, "({} / {})", a, b),
            ArithmeticExpr::PostIncrement(s) => write!(f, "{}++", s),
            ArithmeticExpr::PostDecrement(s) => write!(f, "{}--", s),
        }
    }
}

impl fmt::Display for BooleanExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanExpr::True => write!(f, "true"),
            BooleanExpr::False => write!(f, "false"),
            BooleanExpr::Not(b) => write!(f, "!{}", b),
            BooleanExpr::And(b1, b2) => write!(f, "({} && {})", b1, b2),
            BooleanExpr::Or(b1, b2) => write!(f, "({} || {})", b1, b2),
            BooleanExpr::NumEq(a1, a2) => write!(f, "({} == {})", a1, a2),
            BooleanExpr::NumNotEq(a1, a2) => write!(f, "({} != {})", a1, a2),
            BooleanExpr::NumLt(a1, a2) => write!(f, "({} < {})", a1, a2),
            BooleanExpr::NumGt(a1, a2) => write!(f, "({} > {})", a1, a2),
            BooleanExpr::NumLtEq(a1, a2) => write!(f, "({} <= {})", a1, a2),
            BooleanExpr::NumGtEq(a1, a2) => write!(f, "({} >= {})", a1, a2),
        }
    }
}

impl PartialEq for ArithmeticExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ArithmeticExpr::Number(a), ArithmeticExpr::Number(b)) => a == b,

            (ArithmeticExpr::Variable(a), ArithmeticExpr::Variable(b))
            | (ArithmeticExpr::PostDecrement(a), ArithmeticExpr::PostDecrement(b))
            | (ArithmeticExpr::PostIncrement(a), ArithmeticExpr::PostIncrement(b)) => a == b,

            (ArithmeticExpr::Add(a1, a2), ArithmeticExpr::Add(b1, b2))
            | (ArithmeticExpr::Mul(a1, a2), ArithmeticExpr::Mul(b1, b2)) => {
                (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1)
            }

            (ArithmeticExpr::Sub(a1, a2), ArithmeticExpr::Sub(b1, b2))
            | (ArithmeticExpr::Div(a1, a2), ArithmeticExpr::Div(b1, b2)) => a1 == b1 && a2 == b2,

            _ => false,
        }
    }
}
