use std::os::macos::raw::stat;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::app::http;
use crate::app::params::HttpParamsBuilder;
use crate::init::params::InitParamsBuilder;
use crate::init::utils::init;

mod app;
mod init;

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

        #[arg(long)]
        static_path: Option<String>,
    },
    Init {
        #[arg(long)]
        static_path: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run {
            host,
            port,
            static_path,
        } => {
            let mut params_builder = HttpParamsBuilder::new();

            if let Some(host) = host {
                params_builder
                    .host(host.to_string())
                    .expect("failed to set host");
            }

            if let Some(port) = port {
                params_builder.port(*port).expect("failed to set port");
            }

            if let Some(static_path) = static_path {
                params_builder
                    .static_path(PathBuf::from(static_path))
                    .expect("failed to set static path");
            }

            let params = params_builder.build().expect("failed to build params");
            http::run(params).await;
        }
        Commands::Init { static_path } => {
            let mut params_builder = InitParamsBuilder::new();

            if let Some(static_path) = static_path {
                params_builder
                    .static_path(PathBuf::from(static_path))
                    .expect("failed to set static path");
            }

            let params = params_builder.build().expect("failed to build params");
            init(params).expect("failed to init");
        }
    }
}
