use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::parser::ast::*;

#[derive(Debug)]
pub enum ExpressionTree<T: Domain> {
    Value(T),
    Variable(Identifier, T),
    Binop(
        T,
        ArithmeticExpr,
        Box<ExpressionTree<T>>,
        Box<ExpressionTree<T>>,
    ),
}

impl<T: Domain> ExpressionTree<T> {
    pub fn get_value(&self) -> T {
        match self {
            ExpressionTree::Value(v)
            | ExpressionTree::Variable(_, v)
            | ExpressionTree::Binop(v, _, _, _) => v.clone(),
        }
    }

    pub fn build(expr: &ArithmeticExpr, state: &State<T>) -> (ExpressionTree<T>, State<T>) {
        let (val, new_state) = T::eval_aexpr(expr, state);

        match expr {
            ArithmeticExpr::Number(_) | ArithmeticExpr::Interval(_, _) => {
                (ExpressionTree::Value(val), new_state)
            }

            ArithmeticExpr::Variable(var)
            | ArithmeticExpr::PostIncrement(var)
            | ArithmeticExpr::PostDecrement(var) => {
                (ExpressionTree::Variable(var.to_string(), val), new_state)
            }

            ArithmeticExpr::Add(a1, a2)
            | ArithmeticExpr::Sub(a1, a2)
            | ArithmeticExpr::Mul(a1, a2)
            | ArithmeticExpr::Div(a1, a2) => {
                let (l, new_state) = Self::build(a1, &new_state);
                let (r, new_state) = Self::build(a2, &new_state);
                (
                    ExpressionTree::Binop(val, expr.clone(), Box::new(l), Box::new(r)),
                    new_state,
                )
            }
        }
    }
}
