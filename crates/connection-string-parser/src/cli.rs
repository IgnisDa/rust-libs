use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum Part {
    Scheme,
    Host,
    Port,
    User,
    Password,
    /// Often corresponds to the database name
    Path,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The connection string to parse
    pub url: String,

    /// The part of the connection string to extract
    #[arg(short, long, required = true)]
    pub part: Part,
}
