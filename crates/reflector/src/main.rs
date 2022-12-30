use clap::Parser;
use color_eyre::eyre::Result;
use reflector::Cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    dbg!(&cli);
    Ok(())
}
