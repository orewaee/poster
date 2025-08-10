use clap::{Parser, Subcommand};

use crate::app::http;
use crate::app::params::HttpParamsBuilder;

mod app;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(long)]
        host: Option<String>,

        #[arg(long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { host, port } => {
            let mut params_builder = HttpParamsBuilder::new();

            if let Some(host) = host {
                params_builder
                    .host(host.to_string())
                    .expect("failed to set host");
            }

            if let Some(port) = port {
                params_builder.port(*port).expect("failed to set port");
            }

            let params = params_builder.build().expect("failed to build params");
            http::run(params).await;
        }
    }
}
