use clap::{Parser, Subcommand};
use my_request::*;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    url: String,

    //#[arg(long, short)]
    //endpoint: String,
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
    //let endpoint = args.endpoint;
    let proxy_bool = args.command.is_some();
    let redirect = RedirectPolicy::Custom;
    let coupon = "SIGNUP30";

    let username = "wiener";
    let password = "peter";

    let my_client = MyClient::new(url, proxy_bool, redirect);
    my_client.login(username, password).await?;
    my_client.check_cart().await?;

    for i in 1..=11 {
        my_client.add2cart(2, 10 + i * 3).await?;
        my_client.apply_coupon(coupon).await?;

        let res = my_client.purchase().await?.text().await?;
        let tag_name = "table";
        let class_name = "is-table-numbers";
        let tag_attr = "td";
        let gift_cards = my_client
            .scrape(&res, tag_name, class_name, tag_attr, None)
            .await?;

        my_client.redeem_giftcards(gift_cards).await?;
    }

    my_client.add2cart(1, 1).await?;
    my_client.apply_coupon(coupon).await?;
    let res = my_client.purchase().await?;
    let res_str = res.url().as_str();
    if res_str.contains("?order-confirmed=true") {
        println!("Success!");
    } else {
        println!("Couldnt purchase it..");
    }
    Ok(())
}
