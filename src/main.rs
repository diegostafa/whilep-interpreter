use std::fs;

use crate::{
    abstract_state::State,
    integer::Integer,
    interval::Interval,
    invariant::{Invariant, InvariantOperations},
};

mod abstract_state;
mod ast;
mod cli;
mod concrete_semantics;
mod concrete_state;
mod domain;
mod integer;
mod interval;
mod invariant;
mod lattice;

fn interval_from_bounds(min: Option<i32>, max: Option<i32>) -> interval::Interval {
    match (min, max) {
        (Some(m), Some(n)) => Interval::Range(Integer::Value(m), Integer::Value(n)),
        (Some(m), None) => Interval::Range(Integer::Value(m), Integer::PosInf),
        (None, Some(n)) => Interval::Range(Integer::NegInf, Integer::Value(n)),
        (None, None) => Interval::Range(Integer::NegInf, Integer::PosInf),
    }
}

fn main() {
    let opts = cli::parse_options();
    let source = fs::read_to_string(&opts.source_file).expect("[ERROR] failed to read the source");
    let ast = ast::parse(&source).expect("[ERROR] failed to parse the program");
    let bounds = interval_from_bounds(opts.min_interval, opts.max_interval);

    let astate = State::<Interval>::new();
    let inv = Invariant::<Interval>::new();

    println!("[INFO] Evaluating the abstract semantics");
    let (a_state, a_inv) = State::eval_stmt(ast.clone(), (astate, inv), bounds);

    println!("[INFO] ABSTRACT STATE");
    a_state.pretty_print();

    println!("[INFO] PROGRAM POINTS");
    a_inv.pretty_print();

    println!("[INFO] Building the concrete semantics");
    let c_semantics = concrete_semantics::denote_stmt(ast);

    println!("[INFO] Evaluating the concrete semantics");
    let c_state = c_semantics(concrete_state::empty_state());

    println!("[INFO] CONCRETE STATE");
    c_state.pretty_print();
}
