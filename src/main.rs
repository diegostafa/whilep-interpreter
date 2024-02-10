use std::fs;

use crate::abstract_semantics::invariant::*;
use crate::concrete_semantics::state::*;
use crate::domain::interval::*;
use crate::parser::ast;
use crate::types::integer::*;

mod abstract_semantics;
mod cli;
mod concrete_semantics;
mod domain;
mod parser;
mod types;

fn interval_from_bounds(min: Option<i32>, max: Option<i32>) -> Interval {
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

    println!("[INFO] Evaluating the abstract semantics");
    let (astate, inv) = abstract_semantics::state::State::<Interval>::new()
        .eval_stmt(&ast, &Invariant::<Interval>::new());

    println!("[INFO] ABSTRACT STATE");
    astate.pretty_print();

    println!("[INFO] PROGRAM POINTS");
    inv.pretty_print();

    println!("[INFO] Building the concrete semantics");
    let induced_function = concrete_semantics::denote::denote_stmt(ast);

    println!("[INFO] Evaluating the concrete semantics");
    let c_state = induced_function(concrete_semantics::state::State::new());

    println!("[INFO] CONCRETE STATE");
    c_state.unwrap().pretty_print();
}
