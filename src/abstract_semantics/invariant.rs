use crate::abstract_semantics::state::*;
use crate::domain::domain::*;

pub type Invariant<T> = Vec<State<T>>;

pub trait InvariantOperations<T: Domain>: Sized {
    fn new() -> Self;
    fn back(&self) -> State<T>;
    fn concat(&self, others: &[Self]) -> Self;
    fn append(&self, state: State<T>) -> Self;
}

impl<T: Domain> InvariantOperations<T> for Invariant<T> {
    fn new() -> Self {
        vec![]
    }

    fn back(&self) -> State<T> {
        match self.last() {
            Some(state) => state.clone(),
            None => State::new(),
        }
    }

    fn concat(&self, others: &[Self]) -> Self {
        let mut final_inv = self.clone();
        for inv in others {
            final_inv.extend(inv.clone());
        }
        final_inv
    }

    fn append(&self, state: State<T>) -> Self {
        let mut new = self.clone();
        new.push(state);
        new
    }
}
