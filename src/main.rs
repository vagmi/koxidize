use clap::Parser;
use cli::Cli;
use kube::Client;
use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;
    let cli = Cli::parse();
    cli.run(client).await?;
    Ok(())
}
