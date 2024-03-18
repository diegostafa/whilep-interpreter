use crate::abstract_semantics::state::*;
use crate::domain::domain::*;
use crate::parser::ast::*;

#[derive(Debug)]
pub enum ExpressionTree<T: Domain> {
    Value(T),
    Variable(Identifier, T),
    Binop {
        value: T,
        op: ArithmeticExpr,
        l: Box<ExpressionTree<T>>,
        r: Box<ExpressionTree<T>>,
    },
}

impl<T: Domain> ExpressionTree<T> {
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
                let (l, s) = ExpressionTree::build(a1, &state);
                let (r, _) = ExpressionTree::build(a2, &s);
                (
                    ExpressionTree::Binop {
                        value: val,
                        op: expr.clone(),
                        l: Box::new(l),
                        r: Box::new(r),
                    },
                    new_state,
                )
            }
        }
    }

    pub fn refine(&self, refined_value: T, state: State<T>) -> State<T> {
        match self {
            ExpressionTree::Value(_) => state,
            ExpressionTree::Variable(var, val) => state.put(var, val.glb(&refined_value)),
            ExpressionTree::Binop { value, op, l, r } => {
                let a = l.value();
                let b = r.value();
                let c = value.glb(&refined_value);

                // c is more precise, therefore a,b are more precise too
                let (better_a, better_b) = match op {
                    ArithmeticExpr::Add(_, _) => (c - b, c - a),
                    ArithmeticExpr::Sub(_, _) => (c + b, a - c),
                    ArithmeticExpr::Mul(_, _) => (c / b, c / a),
                    ArithmeticExpr::Div(_, _) => (c * b, a / c),
                    _ => unreachable!(),
                };

                let ltree_state = l.refine(better_a, state);
                let rtree_state = r.refine(better_b, ltree_state);
                rtree_state
            }
        }
    }

    pub fn value(&self) -> T {
        match self {
            ExpressionTree::Value(value)
            | ExpressionTree::Variable(_, value)
            | ExpressionTree::Binop { value, .. } => value.clone(),
        }
    }
}
