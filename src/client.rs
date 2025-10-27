//! Client module for interacting with the Chapa API.
//! This module provides the `ChapaClient` struct, which encapsulates
//! methods for initializing transactions, verifying payments, and retrieving bank information.
//! It leverages the `reqwest` crate for HTTP requests and handles authentication
//! using bearer tokens.
//! ## Example
//! ```rust,no_run
//! use chapa_rust::client::ChapaClient;
//! use chapa_rust::config::ChapaConfigBuilder;
//!
//! let chapa_client = ChapaClient::new("your_secret_key").unwrap();
//! // or using a custom config
//! let config = ChapaConfigBuilder::new().build().unwrap();
//! let chapa_client = ChapaClient::from_config(config).unwrap();
//! ```
//! # Errors
//! Errors encountered during API interactions are represented by the
//! [`ChapaError`] enum.
use std::collections::HashMap;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::{
    config::{ChapaConfig, ChapaConfigBuilder},
    error::{ChapaError, Result},
    models::{
        payment::InitializeOptions,
        response::{GetBanksResponse, InitializeResponse, VerifyResponse},
    },
};

/// Client for interacting with the Chapa API.
/// # Example
/// ```rust,no_run
/// use chapa_rust::client::ChapaClient;
/// let chapa_client = ChapaClient::new("your_secret_key").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ChapaClient {
    http: Client,
    config: ChapaConfig,
}

impl ChapaClient {
    /// Creates a new ChapaClient with the provided secret key.
    pub fn new(secret_key: impl Into<String>) -> Result<Self> {
        let config = ChapaConfigBuilder::new().api_key(secret_key).build()?;
        let http = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { http, config })
    }

    /// Creates a new `ChapaClient` from an existing `ChapaConfig`.
    /// You can build a [`ChapaConfig`] using [`ChapaConfigBuilder`].
    pub fn from_config(config: ChapaConfig) -> Result<Self> {
        let http = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { http, config })
    }

    /// Helper function to convert the default_headers of [ChapaConfig] into a HeaderMap for reqwest requests.
    /// # Errors
    /// Returns an error if any header value is invalid.
    fn build_header(headers: &HashMap<String, String>) -> Result<HeaderMap> {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_key = HeaderName::try_from(key)
                .map_err(|e| ChapaError::InvalidHeaderName(format!("{}: {}", key, e)))?;
            let header_value = HeaderValue::try_from(value)
                .map_err(|e| ChapaError::InvalidHeaderValue(format!("{}: {}", value, e)))?;

            header_map.insert(header_key, header_value);
        }
        Ok(header_map)
    }

    /// Helper function to make a generic GET or POST request to the Chapa API.
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    async fn make_request<T, K>(&self, endpoint: &str, method: &str, body: Option<K>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        K: serde::Serialize,
    {
        let url = format!(
            "{}/{}/{}",
            self.config.base_url, self.config.version, endpoint
        );
        let headers = Self::build_header(&self.config.default_headers)?;
        let method = reqwest::Method::try_from(method)
            .map_err(|e| ChapaError::InvalidHttpMethod(format!("{}: {}", method, e)))?;

        let mut request = self.http.request(method, url);
        if let Some(b) = body {
            request = request.json(&b);
        }
        Ok(request
            .bearer_auth(&self.config.api_key)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    /// Retrieves the list of all banks supported by Chapa.
    ///
    /// This function makes a `GET` request to the `/banks` endpoint and
    /// deserializes the JSON response into a [`GetBanksResponse`] struct.
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::client::ChapaClient;
    /// use chapa_rust::config::ChapaConfigBuilder;
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let banks = client.get_banks().await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_banks(&mut self) -> Result<GetBanksResponse> {
        let response = self
            .make_request::<GetBanksResponse, ()>("banks", "GET", None)
            .await?;

        Ok(response)
    }

    /// Initializes a new transaction with Chapa.
    ///
    /// Sends a `POST` request to `/transaction/initialize` with transaction
    /// details provided in the [`InitializeOptions`] struct.
    ///
    /// # Parameters
    /// - `transaction`: The transaction details (amount, currency, customer info, etc.)
    ///
    /// # Example
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder, models::payment::InitializeOptions, models::bank::Currency};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let transaction = InitializeOptions {
    ///         amount: "100".to_string(),
    ///         currency: Currency::ETB,
    ///         email: Some("customer@gmail.com".to_string()),
    ///         first_name: Some("John".to_string()),
    ///         last_name: Some("Doe".to_string()),
    ///         tx_ref: String::from("some_generated_tax_ref"),
    ///         ..Default::default()
    ///     };
    /// let response = client.initialize_transaction(transaction).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub async fn initialize_transaction(
        &mut self,
        transaction: InitializeOptions,
    ) -> Result<InitializeResponse> {
        let response = self
            .make_request::<InitializeResponse, InitializeOptions>(
                "transaction/initialize",
                "POST",
                Some(transaction),
            )
            .await?;

        Ok(response)
    }

    /// Verifies the status of a transaction using its reference ID.
    ///
    /// This function makes a `GET` request to `/transaction/verify/{tx_ref}`
    /// and returns the transaction’s verification details.
    ///
    /// # Parameters
    /// - `tx_ref`: A unique reference string identifying the transaction.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
    /// dotenvy::dotenv().ok();
    /// let config = ChapaConfigBuilder::new().build().unwrap();
    /// let mut client = ChapaClient::from_config(config).unwrap();
    /// let tx_ref = "your_transaction_reference";
    /// let response = client.verify_transaction(tx_ref).await.unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    pub async fn verify_transaction(&mut self, tx_ref: &str) -> Result<VerifyResponse> {
        let endpoint = format!("transaction/verify/{}", tx_ref);

        let response = self
            .make_request::<VerifyResponse, ()>(endpoint.as_str(), "GET", None)
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, Matcher};

    #[tokio::test]
    async fn test_get_banks() {
        let mut server = mockito::Server::new_async().await;
        let success = server
            .mock("GET", "/v1/banks")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Banks retrieved",
                "data": [
                    {
                        "id": 130,
                        "slug": "abay_bank",
                        "swift": "ABAYETAA",
                        "name": "Abay Bank",
                        "acct_length": 16,
                        "country_id": 1,
                        "is_mobilemoney": null,
                        "is_active": 1,
                        "is_rtgs": 1,
                        "active": 1,
                        "is_24hrs": null,
                        "created_at": "2023-01-24T04:28:30.000000Z",
                        "updated_at": "2024-08-03T08:10:24.000000Z",
                        "currency": "ETB"
                    }
                ]
                        }))
                .unwrap(),
            )
            .create_async()
            .await;

        let failure = server
            .mock("GET", "/v1/banks")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Invalid API Key	",
                "status": "failed",
                "data": null
                }))
                .unwrap(),
            )
            .create_async()
            .await;

        let config = ChapaConfigBuilder::new()
            .base_url(server.url())
            .api_key("CHASECK-xxxxxxxxxxxxxxxx")
            .build()
            .unwrap();
        let mut client = ChapaClient::from_config(config).unwrap();

        // ACT for success
        let response_success = client.get_banks().await.unwrap();
        assert!(!response_success.message.is_null());
        assert!(response_success.data.is_some());

        // ACT for failure
        let response_failure = client.get_banks().await.unwrap();
        assert!(!response_failure.message.is_null());
        // assert_eq!(response_failure.status, "failed");
        assert!(response_failure.data.is_none());

        success.assert_async().await;
        failure.assert_async().await;
    }

    #[tokio::test]
    async fn test_initialize_transaction() {
        let mut server = mockito::Server::new_async().await;
        let success = server
            .mock("POST", "/v1/transaction/initialize")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Hosted Link",
                "status": "success",
                "data": {
                    "checkout_url": "https://checkout.chapa.co/checkout/payment/V38JyhpTygC9QimkJrdful9oEjih0heIv53eJ1MsJS6xG"
                    }
                }))
                .unwrap(),
            )
            .create_async()
            .await;

        let failure = server
            .mock("POST", "/v1/transaction/initialize")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                  "message": "Authorization required	",
                  "status": "failed",
                  "data": null
                }))
                .unwrap(),
            )
            .create_async()
            .await;

        let config = ChapaConfigBuilder::new()
            .base_url(server.url())
            .api_key("CHASECK-xxxxxxxxxxxxxxxx")
            .build()
            .unwrap();
        let mut client = ChapaClient::from_config(config).unwrap();

        let transaction_success = InitializeOptions {
            amount: "100".to_string(),
            currency: crate::models::bank::Currency::ETB,
            email: Some("customer@gmail.com".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            tx_ref: String::from("some_generated_tax_ref"),
            ..Default::default()
        };
        let transaction_failure = InitializeOptions {
            ..Default::default()
        };

        // ACT for success
        let response_success = client
            .initialize_transaction(transaction_success)
            .await
            .unwrap();
        assert_eq!(response_success.status, "success");
        assert!(!response_success.message.is_null());
        assert!(response_success.data.is_some());

        // ACT for failure
        let response_failure = client
            .initialize_transaction(transaction_failure)
            .await
            .unwrap();
        assert_eq!(response_failure.status, "failed");
        assert!(!response_failure.message.is_null());
        assert!(response_failure.data.is_none());

        success.assert_async().await;
        failure.assert_async().await;
    }

    #[tokio::test]
    async fn test_verify_transaction() {
        let mut server = mockito::Server::new_async().await;
        let success = server
            .mock("GET", "/v1/transaction/verify/chewatatest-6669")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Payment details",
                "status": "success",
                "data": {
                    "first_name": "Bilen",
                    "last_name": "Gizachew",
                    "email": "abebech_bekele@gmail.com",
                    "currency": "ETB",
                    "amount": 100,
                    "charge": 3.5,
                    "mode": "test",
                    "method": "test",
                    "type": "API",
                    "status": "success",
                    "reference": "6jnheVKQEmy",
                    "tx_ref": "chewatatest-6669",
                    "customization": {
                        "title": "Payment for my favourite merchant",
                        "description": "I love online payments",
                        "logo": null
                    },
                    "meta": null,
                    "created_at": "2023-02-02T07:05:23.000000Z",
                    "updated_at": "2023-02-02T07:05:23.000000Z"
                  }
                }))
                .unwrap(),
            )
            .create_async()
            .await;

        let failure = server
            .mock("GET", "/v1/transaction/verify/chewatatest-6669")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Invalid transaction or Transaction not found	",
                "status": "failed",
                "data": null
                }))
                .unwrap(),
            )
            .create_async()
            .await;

        let config = ChapaConfigBuilder::new()
            .base_url(server.url())
            .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
            .build()
            .unwrap();
        let mut client = ChapaClient::from_config(config).unwrap();

        // ACT for success
        let response_success = client.verify_transaction("chewatatest-6669").await.unwrap();
        assert_eq!(response_success.status, "success");
        assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
        assert!(response_success.data.is_some());

        // ACT for failure
        let response_failure = client.verify_transaction("chewatatest-6669").await.unwrap();
        assert_eq!(response_failure.status, "failed");
        assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
        assert!(response_failure.data.is_none());

        success.assert_async().await;
        failure.assert_async().await;
    }
}
