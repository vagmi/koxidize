use clap::{Parser, Subcommand};
use kube::Client;
use anyhow::Result;

use crate::cli::deploy_nginx::deploy_nginx_obj;

use self::{list_pods::list_pods, deploy_nginx::deploy_nginx};
mod list_pods;
mod deploy_nginx;

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
            Commands::DeployNginx => {
                let dep = deploy_nginx_obj(client.clone()).await?;
                println!("created deployment {:?} ", dep.metadata.name);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ListPods,
    DeployNginx
}
