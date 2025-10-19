//! Models related to banks and bank listings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the response from Chapa when fetching the list of supported banks.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBanksResponse {
    /// The message of the response.
    pub message: String,
    /// The response data.
    pub data: Vec<Bank>,
}

/// Represents a single bank entry from Chapa’s bank list.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    /// The unique identifier of the bank.
    pub id: u32,
    /// The swift code of the bank.
    pub swift: String,
    /// The name of the bank.
    pub name: String,
    /// The account number length for the bank.
    pub acct_length: u32,
    /// The country identifier for the bank.
    pub country_id: u32,
    /// The creation timestamp of the bank entry.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the bank entry.
    pub updated_at: DateTime<Utc>,
    /// Whether the bank supports RTGS.
    pub is_rtgs: Option<u8>,
    /// Whether the bank supports mobile money.
    pub is_mobilemoney: Option<u8>,
    /// The currency supported by the bank.
    pub currency: Currency,
}

/// Represents the supported currencies for banks.
#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    /// Ethiopian Birr
    ETB,
    /// United States Dollar
    USD,
}
