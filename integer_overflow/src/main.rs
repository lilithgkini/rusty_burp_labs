use clap::{Parser, Subcommand};
use my_request::*;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    url: String,

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
    let url = args.url;
    let proxy_bool = args.command.is_some();
    let redirect = RedirectPolicy::Custom;

    let username = "wiener";
    let password = "peter";

    let myclient = MyClient::new(url, proxy_bool, redirect);
    println!("Trying to login to {} as \"{}\"", myclient.url, username);
    myclient.login(username, password).await?;
    let num_requests = 324;
    println!("Lets send {} requests", num_requests);
    myclient.repeater_serial(num_requests).await;
    println!("Send!");

    println!("Now to finetune..");
    myclient.add2cart(1, 47).await?; //-1221.96

    //myclient.find_final("$5").await?;
    match myclient.find_final("$5").await {
        Ok(()) => (),
        Err(_) => myclient.find_final("$4").await?,
    }

    println!("And finally the purches!");

    let res = myclient.purchase().await?;
    let res_str = res.url().as_str();
    if res_str.contains("?order-confirmed=true") {
        println!("Success!");
    } else {
        println!("Couldnt purchase it..");
    }
    Ok(())
}
