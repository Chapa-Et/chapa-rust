//! Easy to use Rust sdk to integrate chapa to your application
//!
//! set your chapa credentials at the root of the project .env file and you are good to go.
//!

use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::env;

// this module contains types and functions realted with performing transactions
pub mod transaction;

// contains types used for serialzing and deserialzing request and response data and so on.
pub mod types;

fn authorize() -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let api_key = env::var("CHAPA_API_PRIVATE_KEY")?; // NOTE: turbo-fished operation

    let mut headers = HeaderMap::new(); // headers hashmap

    // this casting is necessary because since headers needs a HeaderValue not a string
    let api_key_header_value = format!("Bearer {}", api_key).parse().unwrap();
    let content_type = "application/json".parse().unwrap();

    headers.insert(AUTHORIZATION, api_key_header_value);
    headers.insert(CONTENT_TYPE, content_type);

    return Ok(headers);
}

pub async fn get_banks() -> Result<(), Box<dyn std::error::Error>> {
    let chapa_base_url = env::var("CHAPA_BASE_URL")?;
    let version = env::var("CHAPA_VERSION")?;

    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let banks_url = format!("{}/{}/banks", chapa_base_url, version);
    let response = client.get(banks_url).headers(headers).send().await?;

    // Deserialization into Bank and BankRequestResponse structs
    let response_json = response.json::<types::BankRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}
