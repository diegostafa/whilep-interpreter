use std::collections::HashMap;

pub type State = HashMap<String, i32>;

pub fn create_empty() -> State {
    HashMap::new()
}

pub fn read(state: &State, var: &String) -> i32 {
    *state.get(var).expect("[ERROR] undefined variable")
}

pub fn write(mut state: State, var: &String, val: i32) -> State {
    state.insert(var.to_string(), val);
    state
}
