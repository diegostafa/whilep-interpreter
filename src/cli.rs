use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[arg(short, long = "program", required = true)]
    pub program_file: String,

    #[arg(short, long = "state", default_value = None)]
    pub state_file: Option<String>,
}

pub fn parse_options() -> ProgramOptions {
    ProgramOptions::parse()
}
