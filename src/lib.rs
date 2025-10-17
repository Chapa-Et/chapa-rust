#![allow(warnings)]

pub mod models;

use dotenv::dotenv;
use models::ChapaStructs::*;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::collections::HashMap;
use std::env;

// example function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn authorize() -> Result<HeaderMap, Box<dyn std::error::Error>> {
    dotenv().ok(); // pulling in env vars (should include api key)

    let api_key = env::var("CHAPA_API_PUBLIC_KEY")?; // NOTE: turbo-fished operation

    let mut headers = HeaderMap::new(); // headers hashmap

    // this casting is necessary because since headers needs a HeaderValue not a string
    let api_key_header_value = format!("Bearer {}", api_key).parse().unwrap();

    headers.insert(AUTHORIZATION, api_key_header_value);

    return Ok(headers);
}

#[tokio::main]
pub async fn get_banks() -> Result<(), Box<dyn std::error::Error>> {
    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    // TODO: move base URL and version to .env ?
    // const CHAPA_BASE_URL :&str = env::var("CHAPA_BASE_URL");

    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let banks_url = format!("{}/{}/banks", CHAPA_BASE_URL, version);
    let response = client.get(banks_url).headers(headers).send().await?;

    // Deserialization into Bank and BankRequestResponse structs
    let response_json = response.json::<BankRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}

#[tokio::main]
pub async fn initialize_transaction(
    transaction: Transaction,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Transaction");
    println!("{}", transaction.currency);

    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    // TODO: move base URL and version to .env ?
    // const CHAPA_BASE_URL :&str = env::var("CHAPA_BASE_URL");

    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let init_url = format!("{}/{}/transaction/initialize", CHAPA_BASE_URL, version);

    let response = client
        .post(init_url)
        .headers(headers)
        .form(&transaction)
        .send()
        .await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<InitializeRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}

#[tokio::main]
pub async fn verify_transaction(tx_ref: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Verifying Transaction");

    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    // TODO: move base URL and version to .env ?
    // const CHAPA_BASE_URL :&str = env::var("CHAPA_BASE_URL");

    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let verify_url = format!(
        "{}/{}/transaction/verify/{}",
        CHAPA_BASE_URL, version, tx_ref
    );

    println!("{}", verify_url);

    let response = client.get(verify_url).headers(headers).send().await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<VerifyRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // example test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // TODO: Add tests
}
