use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error;
use serde_json::json;

pub async fn create_tts() -> Result<(), Box<dyn Error>> {
    let url = "https://client.camb.ai/apis/tts";
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_static("YOUR_API_KEY"), // Replace with your API key
    );
    let payload = json!({
        "text": "Hello world",
        "voice_id": 1,
        "language": 40,
        "gender": 1,
        "age": 25
    });
    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).json(&payload).send().await?;
    if response.status().is_success() {
        println!("Response: {}", response.text().await?);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }
    Ok(())
}
