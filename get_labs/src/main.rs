use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod file;
mod labs;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command1: Commands1,
}

#[derive(Subcommand)]
enum Commands1 {
    Scrape {
        #[arg(long, short)]
        url: String,

        #[arg(long, short)]
        endpoint: String,

        #[arg(long, short)]
        file: PathBuf,

        #[command(subcommand)]
        command: Option<Commands>,
    },
    Mystery {
        #[arg(long, short)]
        file: PathBuf,
    },
}

#[derive(Subcommand)]
enum Commands {
    Proxy,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let args = Args::parse();
    let mode = args.command1;

    match mode {
        Commands1::Scrape {
            url,
            endpoint,
            file,
            command,
        } => {
            info!("scraping for all the available categories");
            labs::collect_labs(url, endpoint, file, command.is_some()).await?
        }
        Commands1::Mystery { file } => {
            info!("getting a random Mystery lab..");
            labs::mystery_lab(file).await?
        }
    };

    Ok(())
}
