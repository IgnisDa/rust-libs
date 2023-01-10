use arch_reflector::{run, Cli};
use clap::Parser;
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    run(&cli).await;
    Ok(())
}
