use clap::{Parser, Subcommand};
use kube::Client;
use anyhow::Result;

use self::{
    list_pods::list_pods, 
    deploy_nginx::deploy_nginx_obj, 
    expose_service::expose_nginx_obj,
    port_forward::port_forward
};

mod port_forward;
mod list_pods;
mod deploy_nginx;
mod expose_service;

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
            Commands::ExposeService => {
                let svc = expose_nginx_obj(client.clone()).await?;
                println!("created service {:?} ", svc.metadata.name);
                Ok(())
            }
            Commands::PortForward => {
                port_forward(client.clone()).await?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ListPods,
    DeployNginx,
    ExposeService,
    PortForward
}
