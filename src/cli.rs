use clap::Parser;

use cli_tables::Table;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[arg(
        short,
        long = "source-file",
        required = true,
        help = "Path to the source file"
    )]
    pub source_file: String,

    #[clap(long, action, help = "Perform a concrete evaluation")]
    pub eval: bool,

    #[clap(
        long,
        action,
        help = "Perform an abstract evaluation on the interval domain"
    )]
    pub check_interval: bool,

    #[clap(
        long,
        action,
        help = "Perform an abstract evaluation on the constant domain"
    )]
    pub check_constant: bool,

    #[arg(
        short = 'b',
        long,
        help = "Set the lower and upper bounds for the interval domain"
    )]
    pub bounds: Option<String>,
}

pub fn parse_options() -> ProgramOptions {
    ProgramOptions::parse()
}

pub fn draw_table(headers: Vec<String>, rows: Vec<Vec<String>>) {
    let mut table_data: Vec<Vec<String>> = vec![];

    table_data.push(headers);
    for row in rows {
        table_data.push(row);
    }

    let mut table = Table::new(&table_data);
    println!("{}", table.to_string());
}
