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
        bank::SwapOptions,
        direct_charge::{DirectChargeOptions, DirectChargeType, VerifyDirectChargeOption},
        payment::InitializeOptions,
        response::{
            BulkTransferResponse, DirectChargeResponse, DirectChargeVerifyResponse,
            GetBalancesResponse, GetBanksResponse, GetTransactionsResponse, GetTransfersResponse,
            InitializeResponse, SwapResponse, TransactionLogsResponse, TransferResponse,
            VerifyBulkTransferResponse, VerifyResponse, VerifyTransferResponse,
        },
        transfer::{BulkTransferOptions, TransferOptions},
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

    //
    // ======================================= Bank and Balance related endpoints ===========================
    //

    /// Retrieves the list of all banks supported by Chapa.
    ///
    /// This function makes a `GET` request to the `/banks` endpoint and
    /// deserializes the JSON response into a [`GetBanksResponse`] struct.
    /// # Example
    /// ```
    /// use chapa_rust::client::ChapaClient;
    /// use chapa_rust::config::ChapaConfigBuilder;
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let mut client = ChapaClient::from_config(config).unwrap();
    ///     let banks = client.get_banks().await.unwrap();
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

    /// Retrieves the bank account balance.
    ///
    /// This function makes a `GET` request to the `/balances` endpoint
    /// and deserializes the JSON response into a [`GetBalancesResponse`] struct.
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    ///
    /// Note: Always check your available balance before initiating transfers to ensure you have sufficient funds.
    pub async fn get_balances(&mut self) -> Result<GetBalancesResponse> {
        let response = self
            .make_request::<GetBalancesResponse, ()>("balances", "GET", None)
            .await?;
        Ok(response)
    }

    /// Retrieves the bank account balance for a specific currency.
    ///
    /// This function makes a `GET` request to the `/balances/{currency}` endpoint
    /// and deserializes the JSON response into a [`GetBalancesResponse`] struct.
    /// # Parameters
    /// - `currency`: The currency code (e.g., "ETB", "USD") for which to retrieve the balance.
    /// # Errors
    /// Returns an error if the network request fails or if the response
    /// cannot be deserialized.
    pub async fn get_balances_by_currency(
        &mut self,
        currency: &str,
    ) -> Result<GetBalancesResponse> {
        let endpoint = format!("balances/{}", currency);
        let response = self
            .make_request::<GetBalancesResponse, ()>(endpoint.as_str(), "GET", None)
            .await?;

        Ok(response)
    }

    /// Swaps currencies using the provided swap options.
    ///
    /// Sends a `POST` request to `/swap` with swap details provided in the [`SwapOptions`] struct.
    /// # Parameters
    /// - `swap_options`: The swap details (amount, from currency, to currency)
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be deserialized.
    ///
    /// Note:
    /// > - The minimum amount for conversion is 1 USD
    /// > - Current exchange rate is applied at the time of the swap
    /// > - Swaps are processed immediately and cannot be reversed
    /// > - The maximum allowed amount for swap is $10,000
    /// > - The exchanged amount will be added to the business ETB balance
    pub async fn swap_currencies(&mut self, swap_options: SwapOptions) -> Result<SwapResponse> {
        let response = self
            .make_request::<SwapResponse, SwapOptions>("swap", "POST", Some(swap_options))
            .await?;

        Ok(response)
    }

    //
    // ======================================= Transaction related endpoints ===========================
    //

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
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder, models::payment::InitializeOptions};
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let mut client = ChapaClient::from_config(config).unwrap();
    ///     let transaction = InitializeOptions {
    ///         amount: "100".to_string(),
    ///         currency: "ETB".to_string(),
    ///         email: Some("customer@gmail.com".to_string()),
    ///         first_name: Some("John".to_string()),
    ///         last_name: Some("Doe".to_string()),
    ///         tx_ref: String::from("some_generated_tax_ref"),
    ///         ..Default::default()
    ///     };
    ///     let response = client.initialize_transaction(transaction).await.unwrap();
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
    /// use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
    /// async fn main() {
    ///     dotenvy::dotenv().ok();
    ///     let config = ChapaConfigBuilder::new().build().unwrap();
    ///     let mut client = ChapaClient::from_config(config).unwrap();
    ///     let tx_ref = "your_transaction_reference";
    ///     let response = client.verify_transaction(tx_ref).await.unwrap();
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

    /// Retrieves a list of all transactions.
    ///
    /// This function makes a `GET` request to `/transactions` and returns the list of transactions along with pagination data.
    pub async fn get_transactions(&mut self) -> Result<GetTransactionsResponse> {
        let response = self
            .make_request::<GetTransactionsResponse, ()>("transactions", "GET", None)
            .await?;

        Ok(response)
    }

    /// Retrieves the logs for a specific transaction using its reference ID.
    ///
    /// This function makes a `GET` request to `/transaction/events/{tx_ref}`
    /// and returns the transaction logs.
    /// # Parameters
    /// - `tx_ref`: A unique reference string identifying the transaction.
    pub async fn get_transaction_logs(&mut self, tx_ref: &str) -> Result<TransactionLogsResponse> {
        let endpoint = format!("transaction/events/{}", tx_ref);

        let response = self
            .make_request::<TransactionLogsResponse, ()>(endpoint.as_str(), "GET", None)
            .await?;

        Ok(response)
    }

    //
    //  ================================= Transfer related endpoints =================================
    //

    /// Initiates a bank transfer using the provided transfer options.
    ///
    /// Sends a `POST` request to `/transfers` with transfer details provided in the [`TransferOptions`] struct.
    /// # Parameters
    /// - `options`: The transfer details (account number, bank code, amount, etc.)
    pub async fn transfer(&mut self, options: TransferOptions) -> Result<TransferResponse> {
        let response = self
            .make_request::<TransferResponse, TransferOptions>("transfers", "POST", Some(options))
            .await?;

        Ok(response)
    }

    /// Verifies the status of a bank transfer using its reference ID.
    ///
    /// This function makes a `GET` request to `/transfers/verify/{reference}`
    /// and returns the transfer’s verification details.
    /// # Parameters
    /// - `reference`: A unique reference string identifying the transfer.
    pub async fn verify_transfer(&mut self, reference: &str) -> Result<VerifyTransferResponse> {
        let endpoint = format!("transfers/verify/{}", reference);

        let response = self
            .make_request::<VerifyTransferResponse, ()>(endpoint.as_str(), "GET", None)
            .await?;

        Ok(response)
    }

    /// Initiates a bulk bank transfer using the provided bulk transfer options.
    ///
    /// Sends a `POST` request to `/bulk-transfers` with bulk transfer details provided in the [`BulkTransferOptions`] struct.
    /// # Parameters
    /// - `options`: The bulk transfer details (currency, list of transfers, etc.)
    pub async fn bulk_transfer(
        &mut self,
        options: BulkTransferOptions,
    ) -> Result<BulkTransferResponse> {
        let response = self
            .make_request::<BulkTransferResponse, BulkTransferOptions>(
                "bulk-transfers",
                "POST",
                Some(options),
            )
            .await?;

        Ok(response)
    }

    /// Verifies the status of a bulk bank transfer using its batch ID.
    ///
    /// This function makes a `GET` request to `/transfers?batch_id={batch_id}`
    /// and returns the bulk transfer’s verification details.
    /// # Parameters
    /// - `batch_id`: The unique identifier for the bulk transfer batch.
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    pub async fn verify_bulk_transfer(
        &mut self,
        batch_id: &str,
    ) -> Result<VerifyBulkTransferResponse> {
        let endpoint = format!("transfers?batch_id={}", batch_id);

        let response = self
            .make_request::<VerifyBulkTransferResponse, ()>(endpoint.as_str(), "GET", None)
            .await?;

        Ok(response)
    }

    /// Retrieves a list of all bank transfers.
    ///
    /// This function makes a `GET` request to `/transfers` and returns the list of transfers along with pagination metadata.
    /// # Errors
    /// Returns an error if the request fails or the response cannot be deserialized.
    pub async fn get_transfers(&mut self) -> Result<GetTransfersResponse> {
        let response = self
            .make_request::<GetTransfersResponse, ()>("transfers", "GET", None)
            .await?;

        Ok(response)
    }

    //
    // ======================================= Direct charge endpoints ===========================
    //

    /// Initiates a direct charge using the provided direct charge options.
    ///
    /// Sends a `POST` request to `/charges?type={type}` with direct charge details provided in the [`DirectChargeOptions`] struct.
    /// # Parameters
    /// - `options`: The direct charge details (amount, currency, customer info, payment type, etc.)
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be deserialized.
    pub async fn direct_charge(
        &mut self,
        ty: &DirectChargeType,
        options: DirectChargeOptions,
    ) -> Result<DirectChargeResponse> {
        let endpoint = format!(
            "charges?type={}",
            serde_json::to_string(ty)?.replace("\"", "")
        );
        let response = self
            .make_request::<DirectChargeResponse, DirectChargeOptions>(
                endpoint.as_str(),
                "POST",
                Some(options),
            )
            .await?;

        Ok(response)
    }

    /// Verifies a direct charge using the provided verification options.
    ///
    /// Sends a `POST` request to `/validate?type={type}` with verification details provided in the [`VerifyDirectChargeOption`] struct.
    /// # Parameters
    /// - `options`: The verification details (reference ID, client ID, payment type, etc.)
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be deserialized.
    pub async fn verify_direct_charge(
        &mut self,
        ty: &DirectChargeType,
        options: VerifyDirectChargeOption,
    ) -> Result<DirectChargeVerifyResponse> {
        let endpoint = format!(
            "validate?type={}",
            serde_json::to_string(ty)?.replace("\"", "")
        );
        let response = self
            .make_request::<DirectChargeVerifyResponse, VerifyDirectChargeOption>(
                endpoint.as_str(),
                "POST",
                Some(options),
            )
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement more rigorous unit tests with mocking
}
