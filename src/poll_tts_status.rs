use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error;

pub async fn poll_tts_status(id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://client.camb.ai/apis/tts/{}", id);
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_static("YOUR_API_KEY"), // Replace with your API key
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;
    if response.status().is_success() {
        println!("Response: {}", response.text().await?);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }
    Ok(())
}
