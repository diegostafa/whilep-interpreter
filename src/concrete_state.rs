use std::collections::HashMap;

pub type State = HashMap<String, i32>;

pub fn create_empty() -> State {
    HashMap::new()
}

pub trait IO {
    fn read(&self, var: &String) -> i32;
    fn update(&self, var: &String, val: i32) -> State;
}

impl IO for State {
    fn read(&self, var: &String) -> i32 {
        *self.get(var).expect("[ERROR] undefined variable")
    }

    fn update(&self, var: &String, val: i32) -> State {
        let mut new_state = self.clone();
        new_state.insert(var.to_string(), val);
        new_state
    }
}
