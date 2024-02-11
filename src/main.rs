use std::fs;

use parser::ast::Statement;

use crate::abstract_semantics::invariant::*;
use crate::domain::constant::*;
use crate::domain::domain::*;
use crate::domain::interval::*;
use crate::parser::ast;
use crate::types::integer::*;

mod abstract_semantics;
mod cli;
mod concrete_semantics;
mod domain;
mod parser;
mod types;

fn run_concrete_evaluation(ast: &Statement) {
    use concrete_semantics::denote::*;
    use concrete_semantics::state::*;

    println!("[INFO] Building the concrete semantics");
    let induced_function = denote_stmt(ast.clone());

    println!("[INFO] Evaluating the concrete semantics");
    let state = induced_function(State::new());

    println!("[INFO] CONCRETE STATE");
    state.unwrap().pretty_print();
}

fn run_abstract_analysis<T: Domain>(ast: &Statement) {
    use abstract_semantics::denote::*;
    use abstract_semantics::state::*;

    println!("[INFO] Abstract domain: {}", std::any::type_name::<T>());

    println!("[INFO] Building the abstract semantics");
    let induced_function: StateFunction<T> = denote_stmt(ast.clone());

    println!("[INFO] Evaluating the abstract semantics");
    let (state, inv) = induced_function((State::new(), Invariant::new()));

    println!("[INFO] ABSTRACT STATE");
    state.pretty_print();

    println!("[INFO] INVARIANTS");
    inv.pretty_print();
}

fn main() {
    let opts = cli::parse_options();
    let source = fs::read_to_string(&opts.source_file).expect("[ERROR] failed to read the source");
    let ast = ast::parse(&source).expect("[ERROR] failed to parse the program");

    unsafe {
        LOWER_BOUND = opts
            .min_interval
            .map(|m| Integer::Value(m))
            .unwrap_or(Integer::NegInf);

        UPPER_BOUND = opts
            .max_interval
            .map(|m| Integer::Value(m))
            .unwrap_or(Integer::PosInf);

        match (LOWER_BOUND, UPPER_BOUND) {
            _ if LOWER_BOUND <= UPPER_BOUND => run_abstract_analysis::<Interval>(&ast),
            _ => run_abstract_analysis::<Constant>(&ast),
        }
    }

    run_concrete_evaluation(&ast)
}
