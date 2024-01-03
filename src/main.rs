use std::fs;

mod ast;
mod cli;
mod eval;
mod state;

fn main() {
    let options = cli::parse_options();

    let init_state = match options.state_file {
        Some(file) => state::create_from_file(&file),
        None => state::create_empty(),
    };

    let source = fs::read_to_string(&options.program_file).expect("couldn't read the program file");
    let ast = ast::parse(&source).expect("failed to parse the program");

    let program = eval::make_denotational(ast.clone());
    let final_state = program(init_state);

    println!("{:#?}", ast);
    println!("{:#?}", final_state);
}
