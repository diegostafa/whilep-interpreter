use crate::domain::domain::*;
use crate::parser::ast::*;

#[derive(Debug)]
pub enum ExpressionTree<T: Domain> {
    Value(T),
    Variable(String, T),
    Binop(
        ArithmeticExpr,
        T,
        Box<ExpressionTree<T>>,
        Box<ExpressionTree<T>>,
    ),
}

impl<T: Domain> ExpressionTree<T> {
    pub fn get_value(&self) -> T {
        match self {
            ExpressionTree::Value(v)
            | ExpressionTree::Variable(_, v)
            | ExpressionTree::Binop(_, v, _, _) => v.clone(),
        }
    }
}
