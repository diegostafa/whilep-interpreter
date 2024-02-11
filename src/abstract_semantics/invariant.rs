use crate::abstract_semantics::state::*;
use crate::domain::domain::*;

pub type Invariant<T> = Vec<State<T>>;

pub trait InvariantOperations: Sized {
    fn new() -> Self;
    fn append(&self, others: &[Self]) -> Self;
    fn pretty_print(&self);
}

impl<T: Domain> InvariantOperations for Invariant<T> {
    fn new() -> Self {
        vec![]
    }

    fn append(&self, others: &[Self]) -> Self {
        let mut final_inv = self.clone();
        for inv in others {
            final_inv.extend(inv.clone());
        }
        final_inv
    }

    fn pretty_print(&self) {
        for (i, state) in self.iter().enumerate() {
            print!("{}: ", i);
            state.pretty_print();
        }
    }
}
