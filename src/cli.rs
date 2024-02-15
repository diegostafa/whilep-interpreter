use clap::Parser;

use cli_tables::Table;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramOptions {
    #[arg(short, long = "source-file", required = true)]
    pub source_file: String,

    #[clap(long, action)]
    pub eval: bool,

    #[clap(long, action)]
    pub check_interval: bool,

    #[clap(long, action)]
    pub check_constant: bool,

    #[arg(short = 'b', long)]
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
