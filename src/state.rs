use std::collections::HashMap;
use std::fs;

pub type State = HashMap<String, i32>;

pub fn create_empty() -> State {
    return HashMap::new();
}

pub fn create_from_file(state_file: &str) -> State {
    let content = fs::read_to_string(state_file).expect("couldn't read the state file");
    let mut state = create_empty();

    for line in content.lines() {
        let (k, v) = match line.split_once(" ") {
            Some(s) => s,
            None => panic!("invalid line in state file: {}", line),
        };

        let var = k.to_string();
        let val = v.parse::<i32>().expect("couldn't parse value");

        state.insert(var, val);
    }

    return state;
}
