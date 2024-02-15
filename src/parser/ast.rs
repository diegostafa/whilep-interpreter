use crate::types::integer::*;
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};
use std::fmt;

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
    Add(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Sub(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Mul(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Div(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    PostIncrement(String),
    PostDecrement(String),
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
            Statement::If { cond, s1, s2 } => write!(f, "if {} then {} else {} end", cond, s1, s2),
            Statement::While { cond, body } => write!(f, "while {} do {} done", cond, body),
            Statement::Chain(s1, s2) => write!(f, "{}; {}", s1, s2),
        }
    }
}

impl fmt::Display for ArithmeticExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticExpr::Number(n) => write!(f, "{}", n),
            ArithmeticExpr::Interval(a, b) => write!(f, "[{}, {}]", a, b),
            ArithmeticExpr::Identifier(s) => write!(f, "{}", s),
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

pub fn is_same_aexpr(a1: &ArithmeticExpr, a2: &ArithmeticExpr) -> bool {
    let binop_nodes = |a: ArithmeticExpr| match a {
        ArithmeticExpr::Add(a1, a2) => (a1, a2),
        ArithmeticExpr::Sub(a1, a2) => (a1, a2),
        ArithmeticExpr::Mul(a1, a2) => (a1, a2),
        ArithmeticExpr::Div(a1, a2) => (a1, a2),
        _ => unreachable!(),
    };

    match (a1, a2) {
        (ArithmeticExpr::Number(a), ArithmeticExpr::Number(b)) => a == b,
        (ArithmeticExpr::Identifier(a), ArithmeticExpr::Identifier(b)) => a == b,
        (ArithmeticExpr::PostDecrement(a), ArithmeticExpr::PostDecrement(b)) => a == b,
        (ArithmeticExpr::PostIncrement(a), ArithmeticExpr::PostIncrement(b)) => a == b,

        (ArithmeticExpr::Number(_), _)
        | (_, ArithmeticExpr::Number(_))
        | (ArithmeticExpr::Identifier(_), _)
        | (_, ArithmeticExpr::Identifier(_))
        | (ArithmeticExpr::PostDecrement(_), _)
        | (_, ArithmeticExpr::PostDecrement(_))
        | (ArithmeticExpr::PostIncrement(_), _)
        | (_, ArithmeticExpr::PostIncrement(_))
        | (ArithmeticExpr::Interval(_, _), _)
        | (_, ArithmeticExpr::Interval(_, _)) => false,

        (ArithmeticExpr::Add(_, _), ArithmeticExpr::Add(_, _))
        | (ArithmeticExpr::Mul(_, _), ArithmeticExpr::Mul(_, _)) => {
            let (lhs1, lhs2) = binop_nodes(a1.clone());
            let (rhs1, rhs2) = binop_nodes(a2.clone());
            (is_same_aexpr(&lhs1, &rhs1) && is_same_aexpr(&lhs2, &rhs2))
                || (is_same_aexpr(&lhs1, &rhs2) && is_same_aexpr(&lhs2, &rhs1))
        }

        (ArithmeticExpr::Sub(_, _), ArithmeticExpr::Sub(_, _))
        | (ArithmeticExpr::Div(_, _), ArithmeticExpr::Div(_, _)) => {
            let (lhs1, lhs2) = binop_nodes(a1.clone());
            let (rhs1, rhs2) = binop_nodes(a2.clone());
            is_same_aexpr(&lhs1, &rhs1) && is_same_aexpr(&lhs2, &rhs2)
        }

        _ => false,
    }
}
