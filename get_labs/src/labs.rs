use my_request::*;
use std::path::PathBuf;

use crate::file::*;

pub async fn collect_labs(
    url: String,
    endpoint: String,
    file_path: PathBuf,
    proxy_bool: bool,
) -> anyhow::Result<()> {
    let redirect = RedirectPolicy::All;

    let tag_name = "select";
    let class_name = "form-select.category";
    let tag_attr = "option";
    let attribute = Some("value");

    let my_client = MyClient::new(url, proxy_bool, redirect);
    let res = my_client.get_request(&endpoint, None).await?.text().await?;
    let x = my_client
        .scrape(&res, tag_name, class_name, tag_attr, attribute)
        .await?;
    let contents = x.join("\n");
    //println!("the result it {:#?}", contents);
    write_contents(file_path, contents).await?;
    Ok(())
}

pub async fn mystery_lab(file_path: PathBuf) -> anyhow::Result<()> {
    //let file_path: PathBuf = "categories".into();
    let category = change_file(file_path).await?;

    let endpoint = format!("https://portswigger.net/academy/labs/launchMystery?categoryId={}&level=1&referrer=/web-security/mystery-lab-challenge&onlyCompleted=true",category);
    print!("{endpoint}");
    Ok(())
}
