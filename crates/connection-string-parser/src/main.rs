use clap::Parser;
use color_eyre::eyre::Result;
use connection_string_parser::{run_command, Cli};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let resp = run_command(cli.url, cli.part)?;
    println!("{resp}");
    Ok(())
}
