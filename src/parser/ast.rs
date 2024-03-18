use crate::{max, types::integer::*};
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
        delay: Option<i64>,
    },

    RepeatUntil {
        body: Box<Statement>,
        cond: Box<BooleanExpr>,
        delay: Option<i64>,
    },
}

pub enum ArithmeticExprError {
    DivByZero,
    InvalidIntervalBounds,
    VariableNotFound,
}

#[derive(Debug, Clone)]
pub enum ArithmeticExpr {
    Number(Integer),
    Interval(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
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

impl Statement {
    pub fn get_max_number(&self) -> Option<i64> {
        match self.clone() {
            Statement::Skip => None,
            Statement::Chain(s1, s2) => max!(s1.get_max_number(), s2.get_max_number()),
            Statement::Assignment { var: _, val } => val.get_max_number(),
            Statement::If { cond, s1, s2 } => {
                max!(
                    cond.get_max_number(),
                    s1.get_max_number(),
                    s2.get_max_number()
                )
            }
            Statement::While {
                cond,
                body,
                delay: _,
            } => {
                max!(cond.get_max_number(), body.get_max_number())
            }
            Statement::RepeatUntil {
                body,
                cond,
                delay: _,
            } => {
                max!(cond.get_max_number(), body.get_max_number())
            }
        }
    }
}

impl ArithmeticExpr {
    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (ArithmeticExpr::Number(a), ArithmeticExpr::Number(b)) => a == b,
            (ArithmeticExpr::Variable(a), ArithmeticExpr::Variable(b)) => a == b,

            (ArithmeticExpr::Add(a1, a2), ArithmeticExpr::Add(b1, b2))
            | (ArithmeticExpr::Mul(a1, a2), ArithmeticExpr::Mul(b1, b2)) => {
                (a1.is_same(b1) && a2.is_same(b2)) || (a1.is_same(b2) && a2.is_same(b1))
            }

            (ArithmeticExpr::Sub(a1, a2), ArithmeticExpr::Sub(b1, b2))
            | (ArithmeticExpr::Div(a1, a2), ArithmeticExpr::Div(b1, b2)) => {
                a1.is_same(b1) && a2.is_same(b2)
            }

            _ => false,
        }
    }

    pub fn get_max_number(&self) -> Option<i64> {
        match self.clone() {
            ArithmeticExpr::Number(Integer::Value(n)) => Some(n.abs() + 1),

            ArithmeticExpr::Interval(a1, a2)
            | ArithmeticExpr::Add(a1, a2)
            | ArithmeticExpr::Sub(a1, a2)
            | ArithmeticExpr::Mul(a1, a2)
            | ArithmeticExpr::Div(a1, a2) => {
                max!(a1.get_max_number(), a2.get_max_number())
            }

            _ => None,
        }
    }
}

impl BooleanExpr {
    pub fn negate(&self) -> BooleanExpr {
        match self.clone() {
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

    pub fn get_max_number(&self) -> Option<i64> {
        match self.clone() {
            BooleanExpr::NumEq(a1, a2)
            | BooleanExpr::NumNotEq(a1, a2)
            | BooleanExpr::NumLt(a1, a2)
            | BooleanExpr::NumGt(a1, a2)
            | BooleanExpr::NumLtEq(a1, a2)
            | BooleanExpr::NumGtEq(a1, a2) => max!(a1.get_max_number(), a2.get_max_number()),
            _ => None,
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Skip => write!(f, "skip"),
            Statement::Assignment { var, val } => write!(f, "{} := {}", var, val),
            Statement::Chain(s1, s2) => write!(f, "{}; {}", s1, s2),
            Statement::If { cond, s1, s2 } => write!(f, "if {} then {} else {} end", cond, s1, s2),
            Statement::While { cond, body, .. } => write!(f, "while {} do {} done", cond, body),
            Statement::RepeatUntil { cond, body, .. } => {
                write!(f, "repeat {} until {}", body, cond)
            }
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
