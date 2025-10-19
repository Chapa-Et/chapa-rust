//! Models for Payment operations
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ------------------------------------- Initialize Payment ---------------------------------------------

/// The Request structure for initializing a payment transaction.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InitializeOptions {
    /// The first name of the customer.
    pub first_name: Option<String>,
    /// The last name of the customer.
    pub last_name: Option<String>,
    /// The email address of the customer.
    pub email: Option<String>,
    /// The phone number of the customer.
    pub phone_number: Option<String>,
    /// The currency for the transaction (e.g., "ETB", "USD").
    pub currency: String,
    /// The amount to be charged in the transaction.
    pub amount: f64,
    /// A unique reference for the transaction.
    pub tx_ref: String,
    /// An optional callback URL for transaction updates.
    pub callback_url: Option<String>,
    /// An optional return URL for redirecting after payment.
    pub return_url: Option<String>,
    /// Customization options for the payment interface.
    pub customization: Option<Customization>,
    /// Subaccount details for splitting payments.
    pub subaccounts: Option<Vec<Subaccount>>,
}

/// Represents a subaccount for payment splitting.
#[derive(Debug, Serialize, Deserialize)]
pub struct Subaccount {
    /// The unique identifier of the subaccount.
    pub id: String,
    /// The type of split (e.g., percentage or flat).
    pub split_type: Option<SplitType>,
    /// The value of the split (e.g., percentage value or flat amount).
    pub split_value: Option<f64>,
}

/// Customization options for the payment interface.
#[derive(Debug, Serialize, Deserialize)]
pub struct Customization {
    /// The title to be displayed on the payment interface.
    pub title: Option<String>,
    /// The description to be displayed on the payment interface.
    pub description: Option<String>,
    /// The logo URL to be displayed on the payment interface.
    pub logo: Option<String>,
}

/// Enum representing the type of split for subaccounts.
#[derive(Debug, Serialize, Deserialize)]
pub enum SplitType {
    /// Percentage-based split.
    PERCENTAGE,
    /// Flat amount split.
    FLAT,
}

/// The Response structure received after initializing a payment transaction.
#[derive(Debug, Deserialize)]
pub struct InitializeResponse {
    /// The message from the API.
    pub message: String,
    /// The status of the API response.
    pub status: String,
    /// The data containing the checkout URL.
    pub data: Option<CheckoutURL>,
}

/// Represents the checkout URL provided by Chapa after a successful initialization.
#[derive(Debug, Deserialize)]
pub struct CheckoutURL {
    /// The checkout
    pub checkout_url: String,
}

// ------------------------------------- Verify Payment ---------------------------------------------

/// The Response structure received after verifying a payment transaction.
#[derive(Debug, Deserialize)]
pub struct VerifyResponse {
    /// The message from the API.
    pub message: String,
    /// The status of the API response.
    pub status: String,
    /// The data containing the verification details.
    pub data: VerifyData,
}

/// Represents the detailed data received when verifying a payment transaction.
#[derive(Debug, Deserialize)]
pub struct VerifyData {
    /// The first name of the customer.
    pub first_name: String,
    /// The last name of the customer.
    pub last_name: String,
    /// The email address of the customer.
    pub email: String,
    /// The currency for the transaction (e.g., "ETB", "USD").
    pub currency: String,
    /// The amount to be charged in the transaction.
    pub amount: String,
    /// The charge for the transaction.
    pub charge: String,
    /// The mode of the transaction.
    pub mode: String,
    /// The payment method used in the transaction.
    pub method: String,
    /// The type of the transaction.
    pub r#type: String,
    /// The status of the transaction.
    pub status: String,
    /// The reference for the transaction.
    pub reference: String,
    /// The transaction reference.
    pub tx_ref: String,
    /// The customization details of the transaction.
    pub customization: Customization,
    /// Additional metadata associated with the transaction.
    pub meta: Option<HashMap<String, String>>, // TODO: Adjust the type as needed(could be the map or a specific struct), to my knowledge the type is unknown
    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was last updated.
    pub updated_at: DateTime<Utc>,
}
