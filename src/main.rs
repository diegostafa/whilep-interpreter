use std::fs;

use crate::{abstract_state::State, pretty_print::*};

mod abstract_semantics;
mod abstract_state;
mod ast;
mod cli;
mod concrete_semantics;
mod concrete_state;
mod integer;
mod interval;
mod lattice;
mod pretty_print;

fn main() {
    println!("[INFO] Parsing the arguments");
    let options = cli::parse_options();

    println!("[INFO] Reading the program file {}", &options.source_file);
    let source =
        fs::read_to_string(&options.source_file).expect("[ERROR] failed to read the program file");

    println!("[INFO] Building the AST");
    let ast = ast::parse(&source).expect("[ERROR] failed to parse the program");

    println!("[INFO] Evaluating the abstract semantics");
    let (a_state, a_inv) = abstract_semantics::eval_stmt(
        ast.clone(),
        (
            abstract_state::empty_state(),
            abstract_state::default_invariant(),
        ),
    );
    //let bounds = interval_from_bounds(options.min_interval, options.max_interval);

    println!("[INFO] ABSTRACT STATE");
    a_state.pretty_print();
    println!("[INFO] PROGRAM POINTS");
    a_inv.pretty_print();

    match a_state {
        State::Bottom => println!("\n[INFO] Aborting concrete semantics"),
        _ => {
            println!("[INFO] Building the concrete semantics");
            let c_semantics = concrete_semantics::denote_stmt(ast);
            let c_state = c_semantics(concrete_state::empty_state());

            println!("[INFO] Evaluating the concrete semantics");
            println!("[INFO] CONCRETE STATE");
            c_state.unwrap().pretty_print();
        }
    }
}
