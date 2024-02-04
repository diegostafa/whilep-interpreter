use std::collections::HashMap;

use crate::integer::Integer;

pub type State = HashMap<String, Integer>;

pub trait IO {
    fn read(&self, var: &String) -> Integer;
    fn put(&self, var: &String, val: Integer) -> State;
}

impl IO for State {
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
}

pub fn empty_state() -> State {
    HashMap::new()
}
