use std::fs;

use crate::interval::*;

mod abstract_semantics;
mod abstract_state;
mod ast;
mod cli;
mod concrete_semantics;
mod concrete_state;
mod integer;
mod interval;
mod lattice;

fn main() {
    println!("[INFO] Parsing the arguments");
    let options = cli::parse_options();

    println!("[INFO] Reading the program file {}", &options.source_file);
    let source =
        fs::read_to_string(&options.source_file).expect("[ERROR] failed to read the program file");

    println!("[INFO] Building the AST");
    let ast = ast::parse(&source).expect("[ERROR] failed to parse the program");

    println!("[INFO] Building the abstract semantics ");
    let a_semantics = abstract_semantics::denote_stmt(ast.clone());
    //let interval = interval_from_bounds(options.min_interval, options.max_interval);

    println!("[INFO] Evaluating the abstract semantics");
    let (a_state, points) = a_semantics((
        abstract_state::empty_state(),
        abstract_state::default_invariant(),
    ));

    println!("ABSTRACT STATE\n{:#?}", a_state);
    println!("\n");
    println!("PROGRAM POINTS\n{:#?}", points);

    println!("[INFO] Building the concrete semantics");
    let c_semantics = concrete_semantics::denote_stmt(ast.clone());
    let c_state = c_semantics(concrete_state::empty_state());

    println!("[INFO] Evaluating the concrete semantics");
    println!("CONCRETE STATE\n{:#?}", c_state);
}
