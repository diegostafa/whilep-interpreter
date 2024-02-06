use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

use crate::integer::Integer;

lalrpop_mod!(pub whilep);

pub fn parse(source: &str) -> Result<Statement, ParseError<usize, Token, &'static str>> {
    whilep::StmtParser::new().parse(source)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Skip,
    Chain(Box<Statement>, Box<Statement>),
    Assignment {
        var: String,
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
}

#[derive(Debug, Clone)]
pub enum ArithmeticExpr {
    Number(Integer),
    Interval(Integer, Integer),
    Identifier(String),
    PostIncrement(String),
    PostDecrement(String),
    Add(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Sub(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Mul(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Div(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
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

pub fn desugar_not_bexpr(expr: BooleanExpr) -> BooleanExpr {
    match expr {
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

pub fn negate_bexpr(expr: &BooleanExpr) -> BooleanExpr {
    BooleanExpr::Not(Box::new(expr.clone()))
}
