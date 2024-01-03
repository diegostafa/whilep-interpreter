use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};
lalrpop_mod!(pub whilep);

pub fn parse(source: &str) -> Result<StatementExpr, ParseError<usize, Token, &'static str>> {
    let parser = whilep::SExprParser::new();
    return parser.parse(source);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum StatementExpr {
    Skip,
    Chain(Box<StatementExpr>, Box<StatementExpr>),

    Assignment {
        var: String,
        val: Box<ArithmeticExpr>,
    },

    If {
        cond: Box<BooleanExpr>,
        s1: Box<StatementExpr>,
        s2: Box<StatementExpr>,
    },

    While {
        cond: Box<BooleanExpr>,
        body: Box<StatementExpr>,
    },
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ArithmeticExpr {
    Identifier(String),
    Number(i32),
    Add(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Sub(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Mul(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Div(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum BooleanExpr {
    True,
    False,
    Not(Box<BooleanExpr>),
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    NumEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    NumLtEq(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
}
