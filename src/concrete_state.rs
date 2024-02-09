use std::collections::HashMap;

use crate::integer::Integer;

#[derive(Debug, Clone)]
pub enum State {
    Bottom,
    Just(HashMap<String, Integer>),
}

pub fn empty_state() -> State {
    State::Just(HashMap::new())
}

impl State {
    pub fn read(&self, var: &String) -> Integer {
        match self {
            State::Bottom => panic!("[ERROR] bottom state"),
            State::Just(state) => match state.get(var) {
                Some(val) => *val,
                None => Integer::Value(0),
            },
        }
    }

    pub fn put(&self, var: &String, val: Integer) -> State {
        match self {
            State::Bottom => panic!("[ERROR] bottom state"),
            State::Just(state) => {
                let mut new_state = state.clone();
                new_state.insert(var.to_string(), val);
                State::Just(new_state)
            }
        }
    }

    pub fn pretty_print(&self) {
        match self {
            State::Just(s) => {
                let mut pretty_state = s
                    .iter()
                    .map(|(var, val)| format!("{}: {}", var, val))
                    .collect::<Vec<String>>();

                pretty_state.sort();
                println!("\t{}", pretty_state.join(", "))
            }
            State::Bottom => println!("\tBOTTOM STATE"),
        }
    }
}
