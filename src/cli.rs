use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[arg(short, long = "source-file", required = true)]
    pub source_file: String,

    #[arg(short = 'm', long = "min-interval")]
    pub min_interval: Option<i32>,

    #[arg(short = 'n', long = "max-interval")]
    pub max_interval: Option<i32>,
}

pub fn parse_options() -> ProgramOptions {
    ProgramOptions::parse()
}
