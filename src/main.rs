use std::fs;

use crate::{integer::Integer, interval::Interval};

mod abstract_semantics;
mod abstract_state;
mod ast;
mod cli;
mod concrete_semantics;
mod concrete_state;
mod integer;
mod interval;

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

    let astate = (
        abstract_state::empty_state(),
        abstract_state::default_invariant(),
    );

    println!("[INFO] Evaluating the abstract semantics");
    let (a_state, a_inv) = abstract_semantics::eval_stmt(ast.clone(), astate);

    println!("[INFO] ABSTRACT STATE");
    a_state.pretty_print();

    println!("[INFO] PROGRAM POINTS");
    abstract_state::pretty_print_inv(&a_inv);

    println!("[INFO] Building the concrete semantics");
    let c_semantics = concrete_semantics::denote_stmt(ast);

    println!("[INFO] Evaluating the concrete semantics");
    let c_state = c_semantics(concrete_state::empty_state());

    println!("[INFO] CONCRETE STATE");
    c_state.pretty_print();
}
