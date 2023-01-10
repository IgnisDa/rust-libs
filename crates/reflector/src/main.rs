use clap::Parser;
use color_eyre::eyre::Result;
use reflector::run;
use reflector::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    run(&cli).await;
    Ok(())
}
