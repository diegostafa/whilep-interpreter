use std::fmt::Display;

use crate::{abstract_state::*, domain::*, lattice::*};

pub type Invariant<T: Domain> = Vec<State<T>>;

pub trait InvariantOperations: Sized {
    fn new() -> Self;
    fn append(&self, others: &[Self]) -> Self;
    fn pretty_print(&self);
}

impl<T: Domain + Lattice + Display + Clone + Eq> InvariantOperations for Invariant<T> {
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
        for state in self {
            state.pretty_print();
        }
    }
}
