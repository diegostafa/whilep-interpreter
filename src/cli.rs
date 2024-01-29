use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[arg(short, long = "source-file", required = true)]
    pub source_file: String,

    #[arg(short = 'm', long = "min-interval", default_value_t = i32::MIN)]
    pub min_interval: i32,

    #[arg(short = 'n', long = "max-interval", default_value_t = i32::MAX)]
    pub max_interval: i32,
}

pub fn parse_options() -> ProgramOptions {
    ProgramOptions::parse()
}
