//! Client module for interacting with the Chapa API.
//! This module provides the `ChapaClient` struct, which encapsulates
//! methods for initializing transactions, verifying payments, and retrieving bank information.
//! It leverages the `reqwest` crate for HTTP requests and handles authentication
//! using bearer tokens.
//! ## Example
//! ```rust,no_run
//! use chapa_rust::client::ChapaClient;
//! let chapa_client = ChapaClient::new("your_secret_key").unwrap();
//! ```
//! # Errors
//! Errors encountered during API interactions are represented by the
//! [`ChapaError`](crate::error::ChapaError) enum.
use std::collections::HashMap;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::{
    config::{ChapaConfig, ChapaConfigBuilder},
    error::{ChapaError, Result},
    models::{
        bank::GetBanksResponse,
        payment::{InitializeOptions, InitializeResponse, VerifyResponse},
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
        let http = Client::new();
        Ok(Self { http, config })
    }

    /// Creates a new `ChapaClient` from an existing `ChapaConfig`.
    /// You can build a [`ChapaConfig`] using [`ChapaConfigBuilder`].
    pub fn from_config(config: ChapaConfig) -> Self {
        let http = Client::new();
        Self { http, config }
    }

    /// Helper function to convert the default_headers of [ChapaConfig] into a HeaderMap for reqwest requests.
    /// # Errors
    /// Returns an error if any header value is invalid.
    fn build_header(headers: &HashMap<String, String>) -> Result<HeaderMap> {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_key = HeaderName::try_from(key)
                .map_err(|e| ChapaError::InvalidHeaderValue(format!("{}: {}", key, e)))?;
            let header_value = HeaderValue::try_from(value)
                .map_err(|e| ChapaError::InvalidHeaderName(format!("{}: {}", value, e)))?;

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
    /// deserializes the JSON response into a [`BankRequestResponse`] struct.
    ///
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
    /// details provided in the [`Transaction`] struct.
    ///
    /// # Parameters
    /// - `transaction`: The transaction details (amount, currency, customer info, etc.)
    ///
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
