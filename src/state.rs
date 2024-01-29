use std::collections::HashMap;

pub type State = HashMap<String, i32>;

pub fn create_empty() -> State {
    return HashMap::new();
}

pub fn read(state: &State, var: &String) -> i32 {
    return *state.get(var).expect("[ERROR] undefined variable");
}

pub fn write(state: &State, var: &String, val: i32) -> State {
    let mut new_state = state.clone();
    new_state.insert(var.to_string(), val);
    return new_state;
}
