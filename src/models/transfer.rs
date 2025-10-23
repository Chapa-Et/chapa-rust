//! Models related to bank transfers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the options required to initiate a bank transfer.
#[derive(Debug, Serialize)]
pub struct TransferOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the account holder.
    pub account_name: Option<String>,
    /// The bank account number to which the transfer will be made.
    pub account_number: String,
    /// The amount to be transferred.
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The currency in which the transfer will be made.
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A unique reference for the transfer.
    pub reference: Option<String>,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
}
// -------------------------------------- Verify Transfer -----------------------------------------------
/// Represents the detailed data received when verifying a bank transfer.
#[derive(Debug, Deserialize)]
pub struct VerifyTransferData {
    /// The name of the account holder.
    pub account_name: String,
    /// The bank account number.
    pub account_number: String,
    /// The mobile number of the account holder.
    pub mobile: String,
    /// The currency in which the transfer will be made.
    pub currency: String,
    /// The amount to be transferred.
    pub amount: f64,
    /// The charge for the transfer.
    pub charge: f64,
    /// The mode of the transfer.
    pub mode: String,
    /// The method used for the transfer.
    pub transfer_method: String,
    /// The narration for the transfer.
    pub narration: String,
    /// The unique identifier for the transfer.
    pub chapa_transfer_id: String,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
    /// The name of the recipient's bank.
    pub bank_name: String,
    /// The cross-party reference for the transfer.
    pub cross_party_reference: String,
    /// The IP address from which the transfer was initiated.
    pub ip_address: String,
    /// The status of the transfer.
    pub status: String,
    /// The unique reference for the transfer.
    pub tx_ref: String,
    /// The creation timestamp of the transfer.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the transfer.
    pub updated_at: DateTime<Utc>,
}

// ------------------------------------- Bulk Transfer Options ---------------------------------------------
/// Represents the options required to initiate a bulk bank transfer.
#[derive(Debug, Serialize)]
pub struct BulkTransferOptions {
    /// The title for the bulk transfer batch.
    pub title: String,
    /// The currency for the bulk transfer.
    pub currency: String,
    /// The list of individual transfer options.
    pub bulk_data: Vec<BulkData>,
}

/// Represents a single transfer entry in a bulk transfer request.
/// It is almost similar to `TransferOptions` for a single transfer.
#[derive(Debug, Serialize)]
pub struct BulkData {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the account holder.
    pub account_name: Option<String>,
    /// The bank account number to which the transfer will be made.
    pub account_number: String,
    /// The amount to be transferred.
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A unique reference for the transfer.
    pub reference: Option<String>,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
}

/// Represents the data returned after initiating a bulk transfer.
#[derive(Debug, Deserialize)]
pub struct BulkTransferData {
    /// The unique identifier for the bulk transfer batch.
    pub id: u32,
    /// The creation timestamp of the bulk transfer batch.
    pub created_at: DateTime<Utc>,
}

// ------------------------------------- Verify Bulk Transfer ---------------------------------------------

// NOTE: this uses both [`TransferMeta`] and [`TransfersData`] for its data and meta fields respectively.

// ------------------------------------- Get all Transfers ---------------------------------------------

/// Represents the metadata associated with paginated transfer responses.
#[derive(Debug, Deserialize)]
pub struct TransferMeta {
    /// The current page number.
    pub current_page: u32,
    /// The URL of the first page.
    pub first_page_url: String,
    /// The last page number.
    pub last_page: u32,
    /// The URL of the last page.
    pub last_page_url: String,
    /// The URL of the next page.
    pub next_page_url: Option<String>,
    /// The path of the current page.
    pub path: String,
    /// The number of items per page.
    pub per_page: u32,
    /// The URL of the previous page.
    pub prev_page_url: Option<String>,
    ///?
    pub to: u32, // TODO: clarify what this field represents
    /// The total number of items.
    pub total: u32,
    /// A list of error messages, if any.
    pub error: Value, // TODO: get clarification on the type of this field
}

/// Represents a single transfer entry in the list of transfers.
#[derive(Debug, Deserialize)]
pub struct TransfersData {
    /// The name of the account holder.
    pub account_name: String,
    /// The bank account number.
    pub account_number: String,
    /// The currency for the transfer.
    pub currency: String,
    /// The amount to be transferred.
    pub amount: f64,
    /// The charge for the transfer.
    pub charge: f64,
    /// The type of the transfer.
    pub transfer_type: String,
    /// The Chapa reference for the transfer.
    pub chapa_reference: String,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
    /// The name of the recipient's bank.
    pub bank_name: String,
    /// The bank reference for the transfer.
    pub bank_reference: String,
    /// The status of the transfer.
    pub status: String,
    /// A unique reference for the transfer.
    pub reference: String,
    /// The creation timestamp of the transfer.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the transfer.
    pub updated_at: DateTime<Utc>,
}
