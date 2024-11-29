# Rust Client for Camb.AI APIs

This Rust project provides implementations for interacting with the Camb.AI API. The project includes the following functionalities:

- **Get All Target Languages**
- **Poll for TTS Status**
- **Get All Voices**
- **Create TTS**
- **Create Transcription**

## Folder Structure

```plaintext
rust-camb-ai/
├── src/
│   ├── get_all_target_languages.rs
│   ├── poll_tts_status.rs
│   ├── get_all_voices.rs
│   ├── create_tts.rs
│   ├── create_transcription.rs
│   ├── main.rs
├── Cargo.toml
├── .gitignore
└── README.md

```

## License

This project is owned by **CAMBAI**. All rights reserved. For more information, visit [CAMBAI's official website](https://www.camb.ai/).

This repository provides the Rust implementation of the [CAMBAI API Reference Endpoint](https://docs.camb.ai/api-reference/endpoint/).

### APIKEY

Obtain your apikey [here](https://docs.camb.ai/introduction#api-key)

### MAIN.RS

The main.rs has some commented codes which where used [here](https://github.com/Camb-ai/MARS5-TTS/issues/88) for demonstration purposes. To test any of the Endpoints, navigate to /rust-camb-ai/src and run the command below

```bash
cargo run 
```

For example to test the endpoint of CREATE TTS, in your main.rs, uncomment the function in line 243 From 

``
 // create_tts::create_tts().await.unwrap();
``

to 

``
create_tts::create_tts().await.unwrap();
``

And you can test on the fly 🚀.