//! # Chapa Rust SDK
//!
//! `chapa-rust` is an **unofficial, community-maintained Rust SDK** for interacting with the
//! [Chapa Payments API](https://developer.chapa.co/).  
//! It enables developers to perform operations such as payment initialization, verification,
//! transfers, subaccount management, and more — with full async support built on top of
//! [Tokio](https://tokio.rs) and compatible with popular web frameworks like
//! [Axum](https://docs.rs/axum), [Actix-Web](https://actix.rs), and any Tower-based ecosystem.
//!
//! ---
//!
//! ## Features
//!
//! - Async-first design (using `reqwest` + `tokio`)
//! - Strongly-typed request/response models
//! - [TODO!] Full coverage of Chapa API endpoints
//! - [TODO!] Built-in validation and error handling
//! - [TODO!] Easy integration with web servers (Axum, Actix, Warp)
//! - [TODO!] Optional utilities (e.g. transaction reference generation)
//!
//! ---
//!
//! ## Core API Overview
//!
//! ```rust
//! use chapa_rust::ChapaClient;
//! use chapa_rust::models::InitializeRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ChapaClient::new("YOUR_SECRET_KEY");
//!
//!     let req = InitializeRequest {
//!         amount: "100".to_string(),
//!         currency: "ETB".to_string(),
//!         email: "customer@example.com".to_string(),
//!         first_name: Some("John".to_string()),
//!         last_name: Some("Doe".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let response = client.initialize(req).await?;
//!     println!("Payment URL: {}", response.data.checkout_url);
//!
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## [TODO!] Supported Endpoints
//!
//! | Category | Methods |
//! |-----------|----------|
//! | Transactions | `initialize`, `verify`, `all_transactions`, `transaction_logs` |
//! | Split Payments | `split_payment` |
//! | Banks | `list_banks` |
//! | Subaccounts | `create_subaccount` |
//! | Transfers | `transfer`, `bulk_transfer`, `verify_transfer`, `all_transfers` |
//! | Direct Charges | `direct_charge`, `authorize_direct_charge` |
//! | Utilities | `generate_tx_ref()` |
//!
//! ---
//!
//! ## [TODO!] Error Handling
//!
//! Errors are represented by the [`ChapaError`](crate::error::ChapaError) enum,
//! which encapsulates HTTP, deserialization, and API-level errors.
//!
//! ```rust
//! match client.verify("TX-12345").await {
//!     Ok(res) => println!("Verified payment: {:?}", res),
//!     Err(err) => eprintln!("Error verifying payment: {:?}", err),
//! }
//! ```
//!
//! ---
//!
//! ## Feature Flags [TODO!]
//!
//! - `blocking` — Enables blocking (non-async) client support  
//! - `serde` — Enables serialization and deserialization (enabled by default)  
//! - `logging` — Enables request/response logging (via `tracing` or `log`)  
//!
//! ```toml
//! [dependencies]
//! chapa-rust = { version = "0.1", features = ["serde", "logging"] }
//! ```
//!
//! ---
//!
//! ## Testing and Linting
//!
//! Run quality checks before committing:
//!
//! ```bash
//! make fmt
//! make lint
//! make test
//! ```
//!
//! All warnings are treated as errors via `cargo clippy -- -D warnings`.
//!
//! ---
//!
//! ## Contributing
//!
//! Contributions are welcome!  
//! See [`CONTRIBUTING.md`](https://github.com/Chapa-Et/chapa-rust/chapa-rust/blob/main/CONTRIBUTING.md)
//! for style conventions and documentation guidelines.
//!
//! ---
//!
//! ## License
//!
//! Licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
//! ---
//!
//! ## Notes
//!
//! This crate is **not officially affiliated** with Chapa.  
//! It aims to provide an ergonomic and type-safe developer experience for
//! Rust developers building payment systems in Ethiopia and beyond.
#![deny(missing_docs)]
pub mod config;
pub mod error;
pub mod models;

use models::chapa_models::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

use crate::{
    config::ChapaConfig,
    error::{ChapaError, Result},
};

/// Builds the default_headers of [ChapaConfig] into a HeaderMap for reqwest requests.
/// # Errors
/// Returns an error if any header value is invalid.
fn build_header(headers: HashMap<String, String>) -> Result<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        let header_key = HeaderName::try_from(key.as_str())
            .map_err(|e| ChapaError::InvalidHeaderValue(format!("{}: {}", key, e)))?;
        let header_value = HeaderValue::try_from(value.as_str())
            .map_err(|e| ChapaError::InvalidHeaderName(format!("{}: {}", value, e)))?;

        header_map.insert(header_key, header_value);
    }
    Ok(header_map)
}

/// Retrieves the list of all banks supported by Chapa.
///
/// This function makes a `GET` request to the `/banks` endpoint and
/// deserializes the JSON response into a [`BankRequestResponse`] struct.
///
/// # Errors
/// Returns an error if the network request fails or if the response
/// cannot be deserialized.
pub async fn get_banks() -> Result<BankRequestResponse> {
    let config = ChapaConfig::builder().build()?;
    let headers = build_header(config.default_headers)?;

    // Building client + making request
    let client = reqwest::Client::new();
    let banks_url = format!("{}/{}/banks", config.base_url, config.version);
    let response = client
        .get(banks_url)
        .bearer_auth(config.api_key)
        .headers(headers)
        .send()
        .await?;

    // Deserialization into Bank and BankRequestResponse structs
    let response_json = response.json::<BankRequestResponse>().await?;

    Ok(response_json)
}

/// Initializes a new transaction with Chapa.
///
/// Sends a `POST` request to `/transaction/initialize` with transaction
/// details provided in the [`Transaction`] struct.
///
/// # Parameters
/// - `transaction`: The transaction details (amount, currency, customer info, etc.)
///
/// # Errors
/// Returns an error if the request fails or if the response cannot be parsed.
pub async fn initialize_transaction(transaction: Transaction) -> Result<InitializeRequestResponse> {
    let config = ChapaConfig::builder().build()?;
    let headers = build_header(config.default_headers)?;

    // Building client + making request
    let client = reqwest::Client::new();
    let init_url = format!(
        "{}/{}/transaction/initialize",
        config.base_url, config.version
    );

    let response = client
        .post(init_url)
        .headers(headers)
        .json(&transaction)
        .send()
        .await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<InitializeRequestResponse>().await?;

    Ok(response_json)
}

/// Verifies the status of a transaction using its reference ID.
///
/// This function makes a `GET` request to `/transaction/verify/{tx_ref}`
/// and returns the transaction’s verification details.
///
/// # Parameters
/// - `tx_ref`: A unique reference string identifying the transaction.
///
/// # Errors
/// Returns an error if the request fails or the response cannot be deserialized.
pub async fn verify_transaction(tx_ref: String) -> Result<VerifyRequestResponse> {
    let config = ChapaConfig::builder().build()?;
    let headers = build_header(config.default_headers)?;

    // Building client + making request
    let client = reqwest::Client::new();
    let verify_url = format!(
        "{}/{}/transaction/verify/{}",
        config.base_url, config.version, tx_ref
    );

    let response = client
        .get(verify_url)
        .bearer_auth(config.api_key)
        .headers(headers)
        .send()
        .await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<VerifyRequestResponse>().await?;

    Ok(response_json)
}

#[cfg(test)]
mod tests {}
