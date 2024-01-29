use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

lalrpop_mod!(pub whilep);

pub fn parse(source: &str) -> Result<Statement, ParseError<usize, Token, &'static str>> {
    let parser = whilep::StmtParser::new();
    return parser.parse(source);
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
    Number(i32),
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
    NumEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumLt(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumGt(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumLtEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumGtEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
}
