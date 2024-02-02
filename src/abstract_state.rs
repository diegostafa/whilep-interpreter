use crate::interval_domain::*;

use std::collections::HashMap;

pub type State = Option<HashMap<String, Interval>>;
pub type ProgramPoints = Vec<State>;

pub fn create_empty() -> State {
    Some(HashMap::new())
}

pub fn read(state: &State, var: &String) -> Interval {
    match state {
        Some(s) => *s.get(var).expect("[ERROR] undefined variable"),
        None => todo!(),
    }
}

pub fn write(mut state: State, var: &String, val: Interval) -> State {
    match state {
        Some(mut s) => {
            s.insert(var.to_string(), val);
            Some(s)
        }
        None => todo!(),
    }
}

pub fn concat(slice: &[ProgramPoints]) -> ProgramPoints {
    let mut final_points = vec![];
    for points in slice {
        final_points.extend(points.clone());
    }
    final_points
}

pub trait IO {
    fn read(&self, var: &String) -> Interval;
    fn update(&self, var: &String, val: Interval) -> State;
}

impl IO for State {
    fn read(&self, var: &String) -> Interval {
        match self {
            Some(s) => *s.get(var).expect("[ERROR] undefined variable"),
            None => todo!(),
        }
    }

    fn update(&self, var: &String, val: Interval) -> State {
        let new_state = self.clone();
        match new_state {
            Some(mut s) => {
                s.insert(var.to_string(), val);
                Some(s)
            }
            None => todo!(),
        }
    }
}

pub trait Lattice {
    fn union(&self, other: &Self) -> Self;
    fn intersect(&self, other: &Self) -> Self;
}

impl Lattice for State {
    fn union(&self, other: &Self) -> Self {
        let mut new_state = create_empty();

        match (self, other) {
            (None, None) => todo!(),
            (None, _) => todo!(),
            (_, None) => todo!(),
            (Some(s1), Some(s2)) => todo!(),
        }
    }

    fn intersect(&self, other: &Self) -> Self {
        let mut new_state = create_empty();

        todo!()
    }
}
