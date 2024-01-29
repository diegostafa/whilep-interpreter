use std::fs;

mod ast;
mod cli;
mod concrete;
mod state;

fn main() {
    println!("[INFO] Parsing the arguments");
    let options = cli::parse_options();

    println!("[INFO] Reading the program file {}", &options.source_file);
    let source =
        fs::read_to_string(&options.source_file).expect("[ERROR] failed to read the program file");

    println!("[INFO] Building the AST");
    let ast = ast::parse(&source).expect("[ERROR] failed to parse the program");

    println!("[INFO] Building the concrete semantics");
    let induced_function = concrete::denote_statement(ast);

    println!("[INFO] Evaluating the concrete semantics");
    println!("{:#?}", induced_function(state::create_empty()));

    let m: i32 = options.min_interval;
    let n: i32 = options.max_interval;
    println!("[INFO] Testing the interval [{}, {}]", m, n);
}
