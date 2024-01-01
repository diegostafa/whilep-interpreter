use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::fs;

mod ast;

lalrpop_mod!(pub whilep);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ExecutionContext {
    #[arg(short = 'S', long, required = true)]
    source_file: String,

    #[arg(short = 's', long, default_value = "")]
    state_file: String,
}

fn main() {
    let context = ExecutionContext::parse();
    let program = fs::read_to_string(context.source_file).expect("failed to read the source file");
    let parser = whilep::SExprParser::new();
    let ast = parser.parse(&program).expect("failed to parse the program");

    println!("{:#?}", ast);
}
