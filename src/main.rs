use std::path::PathBuf;

use clap::{Parser, Subcommand};
use sqlx::sqlite::SqlitePoolOptions;

use crate::app::http;
use crate::app::params::HttpParamsBuilder;
use crate::init::params::InitParamsBuilder;
use crate::init::utils::init;
use crate::post::entity::PostId;
use crate::post::store::{PostStore, SqlitePostStore};

mod app;
mod init;
mod post;
mod session;

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

        #[arg(long)]
        database_url: Option<String>,
    },
    Init {
        #[arg(long)]
        static_path: Option<String>,

        #[arg(long)]
        database_path: Option<String>,
    },
    Create {
        #[arg(long)]
        id: Option<PostId>,

        #[arg(long)]
        password: Option<String>,
    },
    Delete {
        #[arg(long)]
        id: PostId,
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
            database_url,
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

            if let Some(database_url) = database_url {
                params_builder
                    .database_url(database_url.to_string())
                    .expect("failed to set database url");
            }

            let params = params_builder.build().expect("failed to build params");
            http::run(params).await;
        }
        Commands::Init {
            static_path,
            database_path,
        } => {
            let mut params_builder = InitParamsBuilder::new();

            if let Some(static_path) = static_path {
                params_builder
                    .static_path(PathBuf::from(static_path))
                    .expect("failed to set static path");
            }

            if let Some(database_path) = database_path {
                params_builder
                    .database_path(PathBuf::from(database_path))
                    .expect("failed to set database path");
            }

            let params = params_builder.build().expect("failed to build params");
            init(params).expect("failed to init");
        }
        Commands::Create { id, password } => {
            let database_url =
                std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

            let pool = match SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => {
                    eprintln!("failed to connect to database: {}", e);
                    return;
                }
            };

            let post_store = SqlitePostStore::new(pool)
                .await
                .expect("failed to create sqlite repository");

            let id = post_store
                .create(id.clone(), password.clone())
                .await
                .unwrap();

            println!("id of created post: {}", id);
        }
        Commands::Delete { id } => {
            let database_url =
                std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

            let pool = match SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => {
                    eprintln!("failed to connect to database: {}", e);
                    return;
                }
            };

            let post_store = SqlitePostStore::new(pool)
                .await
                .expect("failed to create sqlite repository");

            let success = post_store.delete_by_id(id.clone()).await.unwrap();
            if success {
                println!("post {id} deleted successfully");
            } else {
                println!("failed to delete post {id}");
            }
        }
    }
}
