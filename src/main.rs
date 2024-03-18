use crate::domain::constant::*;
use crate::domain::domain::*;
use crate::domain::interval::*;
use crate::parser::ast::*;
use crate::parser::program_point::*;
use crate::types::integer::*;
use cli::*;
use std::fs;
use std::str::FromStr;

mod abstract_semantics;
mod cli;
mod concrete_semantics;
mod domain;
mod parser;
mod types;
mod utils;

fn run_concrete(ast: &Statement) {
    use concrete_semantics::denote::*;
    use concrete_semantics::state::*;

    let induced_function = denote_stmt(ast.clone());

    println!("[INFO] evaluating the concrete semantics");
    let state = induced_function(State::new());
    match state {
        None => println!("[ERROR] Arithmetic expression error"),
        Some(state) => {
            let headers = vec!["#".to_string(), "Var".to_string(), "Val".to_string()];
            let rows = state
                .iter()
                .enumerate()
                .map(|(i, (k, v))| vec![i.to_string(), k.to_string(), v.to_string()])
                .collect::<Vec<_>>();

            draw_table(headers, rows)
        }
    }
}

fn run_abstract<T: Domain>(ast: &Statement) {
    use abstract_semantics::denote::*;
    use abstract_semantics::state::*;

    let induced_function: StateFunction<T> = denote_stmt(ast.clone());

    println!(
        "[INFO] evaluating the abstract semantics in the {} domain",
        std::any::type_name::<T>().split("::").last().unwrap()
    );
    let (_, inv) = induced_function(State::new());
    let points = get_program_points(ast.clone());

    assert!(inv.len() == points.len());

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

fn set_min_max_interval(opts: &ProgramOptions, ast: &Statement) {
    unsafe {
        match opts.bounds.clone() {
            None => (),
            Some(b) if b == "auto" => {
                let max = ast
                    .get_max_number()
                    .map(|n| Integer::Value(n))
                    .unwrap_or(Integer::PosInf);

                LOWER_BOUND = -max;
                UPPER_BOUND = max;
            }
            Some(b) => match Interval::from_str(&b).unwrap() {
                Interval::Empty => unreachable!(),
                Interval::Range(min, max) if min <= max => {
                    LOWER_BOUND = min;
                    UPPER_BOUND = max;
                }
                _ => panic!("[ERROR] invalid bounds: min > max "),
            },
        }
        println!(
            "[INFO] using interval bounds: [{}, {}]",
            LOWER_BOUND, UPPER_BOUND
        );
    }
}

fn main() {
    let opts = cli::parse_options();
    let source = fs::read_to_string(&opts.source_file).expect("[ERROR] failed to read the source");
    let ast = parse(&source).expect("[ERROR] failed to parse the program");

    if opts.check_interval {
        set_min_max_interval(&opts, &ast);
        run_abstract::<Interval>(&ast);
    }

    if opts.check_constant {
        run_abstract::<Constant>(&ast);
    }

    if opts.eval {
        run_concrete(&ast);
    }
}
