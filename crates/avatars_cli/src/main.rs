use avatars_cli::{run_command, Cli};
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let resp = run_command(cli.seed_string, cli.mood, cli.gender)?;
    print!("{resp}");
    Ok(())
}
