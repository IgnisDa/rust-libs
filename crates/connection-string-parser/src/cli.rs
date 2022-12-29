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
#[command(
    about,
    author,
    version,
    propagate_version = true,
    next_line_help = false,
    disable_help_subcommand = true,
    after_long_help = "This tool can be used to extract parts of a connection string. It is useful for extracting the database name from a connection string, for example."
)]
pub struct Cli {
    /// The connection string to parse
    pub url: String,

    /// The part of the connection string to extract
    #[arg(short, long, required = true)]
    pub part: Part,
}
