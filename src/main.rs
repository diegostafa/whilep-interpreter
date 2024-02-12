use std::fs;

use cli::*;

use crate::abstract_semantics::invariant::*;
use crate::domain::constant::*;
use crate::domain::domain::*;
use crate::domain::interval::*;
use crate::parser::ast::*;
use crate::parser::program_point::*;
use crate::types::integer::*;

mod abstract_semantics;
mod cli;
mod concrete_semantics;
mod domain;
mod parser;
mod types;

fn run_concrete(ast: &Statement) {
    use concrete_semantics::denote::*;
    use concrete_semantics::state::*;

    println!("[INFO] building the concrete semantics");
    let induced_function = denote_stmt(ast.clone());

    println!("[INFO] evaluating the concrete semantics");
    let state = induced_function(State::new());

    println!("[INFO] concrete state");
    state.unwrap().pretty_print();
}

fn run_abstract<T: Domain>(ast: &Statement) {
    use abstract_semantics::denote::*;
    use abstract_semantics::state::*;

    println!("[INFO] building the abstract semantics");
    let induced_function: StateFunction<T> = denote_stmt(ast.clone());

    println!("[INFO] evaluating the abstract semantics");
    let (_, inv) = induced_function((State::new(), Invariant::new()));
    let points = get_program_points(ast.clone());

    assert!(inv.len() == points.len());

    println!("[INFO] invariants");
    let headers = vec![
        "#".to_string(),
        "Program point".to_string(),
        "Invariant".to_string(),
    ];

    let rows = points
        .iter()
        .zip(inv.iter())
        .enumerate()
        .map(|(i, (p, s))| vec![i.to_string(), p.to_string(), s.to_string()])
        .collect::<Vec<_>>();

    draw_table(headers, rows)
}

fn set_min_max_interval(opts: &ProgramOptions) {
    unsafe {
        LOWER_BOUND = opts
            .min
            .map(|m| Integer::Value(m))
            .unwrap_or(Integer::NegInf);

        UPPER_BOUND = opts
            .max
            .map(|m| Integer::Value(m))
            .unwrap_or(Integer::PosInf);
    }
}

fn main() {
    let opts = cli::parse_options();
    let source = fs::read_to_string(&opts.source_file).expect("[ERROR] failed to read the source");
    let ast = parse(&source).expect("[ERROR] failed to parse the program");
    set_min_max_interval(&opts);

    if opts.check {
        if opts.constant {
            run_abstract::<Constant>(&ast);
        }

        if opts.interval {
            run_abstract::<Interval>(&ast);
        }
    }

    if opts.eval {
        run_concrete(&ast);
    }
}
