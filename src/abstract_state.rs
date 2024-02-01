use crate::abstract_domain::*;

use std::collections::HashMap;

pub type State = HashMap<String, Interval>;
pub type ProgramPoints = Vec<State>;

pub fn create_empty() -> State {
    HashMap::new()
}

pub fn read(state: &State, var: &String) -> Interval {
    *state.get(var).expect("[ERROR] undefined variable")
}

pub fn write(mut state: State, var: &String, val: Interval) -> State {
    state.insert(var.to_string(), val);
    state
}

pub fn union(state1: &State, state2: &State) -> State {
    todo!()
}

pub fn concat(slice: &[ProgramPoints]) -> ProgramPoints {
    let mut final_points = vec![];
    for points in slice {
        final_points.extend(points.clone());
    }
    final_points
}
