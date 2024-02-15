use crate::types::integer::*;
use std::collections::HashMap;
pub type State = HashMap<String, Integer>;

pub trait StateOperations {
    fn new() -> State;
    fn read(&self, var: &String) -> Integer;
    fn put(&self, var: &String, val: Integer) -> State;
    fn pretty_print(&self);
}

impl StateOperations for State {
    fn new() -> State {
        HashMap::new()
    }

    fn read(&self, var: &String) -> Integer {
        match self.get(var) {
            Some(val) => *val,
            None => Integer::Value(0),
        }
    }

    fn put(&self, var: &String, val: Integer) -> State {
        let mut new_state = self.clone();
        new_state.insert(var.to_string(), val);
        new_state
    }

    fn pretty_print(&self) {
        let mut pretty_state = self
            .iter()
            .map(|(var, val)| format!("{}: {}", var, val))
            .collect::<Vec<String>>();

        pretty_state.sort();
        println!("{}", pretty_state.join(", "));
    }
}
