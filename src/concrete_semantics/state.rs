use crate::{types::integer::*, ArithmeticExprError, Identifier};
use std::collections::HashMap;

pub type State = HashMap<Identifier, Integer>;

pub trait StateOperations {
    fn new() -> State;
    fn read(&self, var: &Identifier) -> Result<Integer, ArithmeticExprError>;
    fn put(&self, var: &Identifier, val: Integer) -> State;
    fn pretty_print(&self);
}

impl StateOperations for State {
    fn new() -> State {
        HashMap::new()
    }

    fn read(&self, var: &Identifier) -> Result<Integer, ArithmeticExprError> {
        match self.get(var) {
            Some(val) => Ok(*val),
            None => Err(ArithmeticExprError::VariableNotFound),
        }
    }

    fn put(&self, var: &Identifier, val: Integer) -> State {
        let mut new_state = self.clone();
        new_state.insert(var.to_string(), val);
        new_state
    }

    fn pretty_print(&self) {
        if self.is_empty() {
            println!("empty state");
            return;
        }

        let mut pretty_state = self
            .iter()
            .map(|(var, val)| format!("{}: {}", var, val))
            .collect::<Vec<String>>();

        pretty_state.sort();
        println!("{}", pretty_state.join(", "));
    }
}
