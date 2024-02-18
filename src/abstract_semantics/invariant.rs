use crate::abstract_semantics::state::*;
use crate::domain::domain::*;

pub type Invariant<T> = Vec<State<T>>;

pub trait InvariantOperations<T: Domain>: Sized {
    fn back(&self) -> State<T>;
    fn append(&mut self, state: State<T>) -> &Self;
}

impl<T: Domain> InvariantOperations<T> for Invariant<T> {
    fn back(&self) -> State<T> {
        match self.last() {
            Some(state) => state.clone(),
            None => State::new(),
        }
    }

    fn append(&mut self, state: State<T>) -> &Self {
        self.push(state);
        self
    }
}

pub fn concat<T: Domain>(others: &[Invariant<T>]) -> Invariant<T> {
    let mut final_inv = Invariant::new();
    for inv in others {
        final_inv.extend(inv.clone());
    }
    final_inv
}
