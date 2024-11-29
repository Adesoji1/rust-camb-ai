// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use reqwest::Client;
// use serde_json::json;
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the API endpoint
//     let url = "https://client.camb.ai/apis/tts";

//     // Define the payload
//     let payload = json!({
//         "voice_id": 20299,
//         "text": "Hello world",
//         "language": 40,
//         "gender": 1, // Male
//         "age": 43,
//     });

//     // Create the headers
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("x-api-key"),
//         HeaderValue::from_static(""), // Replace with your API key
//     );
//     headers.insert(
//         HeaderName::from_static("content-type"),
//         HeaderValue::from_static("application/json"),
//     );

//     // Create an HTTP client
//     let client = Client::new();

//     // Make the POST request
//     let response = client
//         .post(url)
//         .headers(headers) // Attach the headers
//         .json(&payload) // Attach the JSON payload
//         .send()
//         .await?;

//     // Handle the response
//     if response.status().is_success() {
//         let response_text = response.text().await?;
//         println!("Response: {}", response_text);
//     } else {
//         let error_text = response.text().await?;
//         eprintln!("Error: {}", error_text);
//     }

//     Ok(())
// }


// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the API endpoint
//     let url = "https://client.camb.ai/apis/list_voices";

//     // Create the headers
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("x-api-key"),
//         HeaderValue::from_static(""), // Replace with your API key
//     );

//     // Create an HTTP client
//     let client = reqwest::Client::new();

//     // Make the GET request
//     let response = client.get(url).headers(headers).send().await?;

//     // Handle the response
//     if response.status().is_success() {
//         let response_text = response.text().await?;
//         println!("Response: {}", response_text);
//     } else {
//         let error_text = response.text().await?;
//         eprintln!("Error: {}", error_text);
//     }

//     Ok(())
// }


// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use reqwest::multipart::{Form, Part};
// use std::error::Error;
// use std::fs;
// use std::path::Path;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the API endpoint
//     let url = "https://client.camb.ai/apis/create_transcription";

//     // Create the headers
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("x-api-key"),
//         HeaderValue::from_static(""), // Replace with your API key
//     );

//     // Path to the file
//     let file_path = "/home/adesoji/Music/1.mp3"; // Replace with your file path

//     // Validate file extension and determine MIME type
//     let _allowed_extensions = ["wav", "mp3"];
//     let file_extension = Path::new(file_path)
//         .extension()
//         .and_then(|ext| ext.to_str())
//         .unwrap_or("")
//         .to_lowercase();

//     let mime_type = match file_extension.as_str() {
//         "wav" => "audio/wav",
//         "mp3" => "audio/mpeg",
//         _ => {
//             eprintln!("Error: Unsupported file type. Only .wav and .mp3 are allowed.");
//             return Ok(());
//         }
//     };

//     // Read the file and create a multipart `Part`
//     let file_content = fs::read(file_path)?;
//     let file_part = Part::bytes(file_content)
//         .file_name(format!("file.{}", file_extension)) // Set the filename dynamically
//         .mime_str(mime_type)?; // Set the MIME type dynamically

//     // Create the multipart form
//     let form = Form::new()
//         .text("language", "5") // Replace "5" with the desired language code
//         .part("file", file_part); // Attach the file

//     // Create an HTTP client
//     let client = reqwest::Client::new();

//     // Make the POST request
//     let response = client
//         .post(url)
//         .headers(headers)
//         .multipart(form)
//         .send()
//         .await?;

//     // Handle the response
//     if response.status().is_success() {
//         let response_text = response.text().await?;
//         println!("Response: {}", response_text);
//     } else {
//         let error_text = response.text().await?;
//         eprintln!("Error: {}", error_text);
//     }

//     Ok(())
// }



// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the API endpoint
//     let url = "https://client.camb.ai/apis/target_languages";

//     // Create the headers
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("x-api-key"),
//         HeaderValue::from_static(""), // Replace with your API key
//     );

//     // Create an HTTP client
//     let client = reqwest::Client::new();

//     // Make the GET request
//     let response = client.get(url).headers(headers).send().await?;

//     // Handle the response
//     if response.status().is_success() {
//         let response_text = response.text().await?;
//         println!("Response: {}", response_text);
//     } else {
//         let error_text = response.text().await?;
//         eprintln!("Error: {}", error_text);
//     }

//     Ok(())
// }

// use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the API endpoint with a placeholder for the ID
//     let base_url = "https://client.camb.ai/apis/tts/";
//     let id = "40"; // Replace with the actual ID
//     let url = format!("{}{}", base_url, id);

//     // Create the headers
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         HeaderName::from_static("x-api-key"),
//         HeaderValue::from_static("xxxxxx"), // Replace with your API key
//     );

//     // Create an HTTP client
//     let client = reqwest::Client::new();

//     // Make the GET request
//     let response = client.get(&url).headers(headers).send().await?;

//     // Handle the response
//     if response.status().is_success() {
//         let response_text = response.text().await?;
//         println!("Response: {}", response_text);
//     } else {
//         let error_text = response.text().await?;
//         eprintln!("Error: {}", error_text);
//     }

//     Ok(())
// }

mod get_all_target_languages;
mod poll_tts_status;
mod get_all_voices;
mod create_tts;
mod create_transcription;

#[tokio::main]
async fn main() {
    // Call the function you want to test by uncommenting below
    // get_all_target_languages::get_all_target_languages().await.unwrap();
    // poll_tts_status::poll_tts_status("40").await.unwrap();
    // get_all_voices::get_all_voices().await.unwrap();
    // create_tts::create_tts().await.unwrap();
    // create_transcription::create_transcription("path/to/your/file.wav").await.unwrap();
}
