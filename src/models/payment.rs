//! Models for Payment operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: check the type of `amount` field has some inconsistency in the docs, sometimes it's string sometimes number
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
    pub amount: String,
    /// A unique reference for the transaction.
    pub tx_ref: String,
    /// An optional callback URL for transaction updates.
    pub callback_url: Option<String>,
    /// An optional return URL for redirecting after payment.
    pub return_url: Option<String>,
    /// Customization options for the payment interface.
    pub customization: Option<Customization>,
    /// Additional metadata to be associated with the transaction.
    pub meta: serde_json::Value, // NOTE: Using serde_json::Value to allow flexible metadata structure, but if the structure is known, consider using a specific struct or HashMap<String, String>
                                 //? The server seems to ignore the field below for now, it returns 400 Bad Request if included. I took it from the Node.js SDK.
                                 // pub subaccounts: Option<Vec<Subaccount>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The title to be displayed on the payment interface.
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The description to be displayed on the payment interface.
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Represents the checkout URL provided by Chapa after a successful initialization.
#[derive(Debug, Deserialize)]
pub struct CheckoutURL {
    /// The checkout
    pub checkout_url: String,
}

// ------------------------------------- Verify Payment ---------------------------------------------

/// Represents the detailed data received when verifying a payment transaction.
// TODO: Adjust field types as needed based on actual API response, I made most optional to avoid deserialization issues
#[derive(Debug, Deserialize)]
pub struct VerifyPaymentData {
    /// The first name of the customer.
    pub first_name: Option<String>,
    /// The last name of the customer.
    pub last_name: Option<String>,
    /// The email address of the customer.
    pub email: Option<String>,
    /// The currency for the transaction (e.g., "ETB", "USD").
    pub currency: Option<String>,
    /// The amount to be charged in the transaction.
    pub amount: f64,
    /// The charge for the transaction.
    pub charge: Option<f64>, // TODO: sometimes works with Option<String> but the documentation is floating point. check it out.
    /// The mode of the transaction.
    pub mode: Option<String>,
    /// The payment method used in the transaction.
    pub method: Option<String>,
    /// The type of the transaction.
    pub r#type: Option<String>,
    /// The status of the transaction.
    pub status: Option<String>,
    /// The reference for the transaction.
    pub reference: Option<String>,
    /// The transaction reference.
    pub tx_ref: Option<String>,
    /// The customization details of the transaction.
    pub customization: Option<Customization>,
    /// Additional metadata associated with the transaction.
    pub meta: Option<String>, // TODO: Adjust the type as needed(could be the map or a specific struct), to my knowledge the type is not documented
    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was last updated.
    pub updated_at: DateTime<Utc>,
}

// ------------------------------------- Transaction Timeline ---------------------------------------------

/// Represents a single event in the transaction timeline.
#[derive(Debug, Deserialize)]
pub struct TransactionLog {
    /// The unique identifier of the transaction log item.
    pub item: u32,
    /// The message associated with the transaction event.
    pub message: String,
    #[serde(rename = "type")]
    /// The type of the transaction event.
    pub event_type: String,
    /// The timestamp when the transaction event was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction event was last updated.
    pub updated_at: DateTime<Utc>,
}
