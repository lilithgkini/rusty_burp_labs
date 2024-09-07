use std::path::PathBuf;
use tokio::fs::read_to_string;

pub async fn get_file_contents(file_path: PathBuf) -> anyhow::Result<String> {
    let contents = read_to_string(file_path).await?;
    let result = contents.trim().to_string();
    Ok(result)
}

pub fn extract_replace_data(
    endpoint: &str,
    payload: Option<&str>,
    from: &str,
    to: &str,
) -> (String, Option<String>) {
    let (endpoint, payload) = match payload {
        Some(payload) => (endpoint.to_string(), Some(payload.replace(from, to))),
        None => (endpoint.replace(from, to), None),
    };
    (endpoint, payload)
}
