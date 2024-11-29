use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use std::error::Error;
use std::fs;

pub async fn create_transcription(file_path: &str) -> Result<(), Box<dyn Error>> {
    let url = "https://client.camb.ai/apis/create_transcription";
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_static("YOUR_API_KEY"), // Replace with your API key
    );

    let file_content = fs::read(file_path)?;
    let file_part = Part::bytes(file_content)
        .file_name("file.wav") // Update filename dynamically based on extension
        .mime_str("audio/wav")?; // Adjust MIME type if needed

    let form = Form::new().text("language", "5").part("file", file_part);
    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).multipart(form).send().await?;
    if response.status().is_success() {
        println!("Response: {}", response.text().await?);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }
    Ok(())
}
