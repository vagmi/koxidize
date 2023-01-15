use clap::{Parser, Subcommand};
use kube::Client;
use anyhow::Result;

use self::list_pods::list_pods;
mod list_pods;

#[derive(Debug, Parser)]
#[command(name="koxide")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

impl Cli {
    pub async fn run(&self, client: Client) -> Result<()> {
        match self.command {
           Commands::ListPods => {
                let pods = list_pods(client.clone()).await?;

                pods.iter().for_each(|p| {
                    println!("Found pod {:?}", p.metadata.name);
                });
                Ok(())
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ListPods
}
