//! Models related to banks and bank listings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

// ---------------------- Balance Models ----------------------

/// Represents the balance data returned from Chapa’s bank balance endpoint.
#[derive(Debug, Deserialize)]
pub struct BalanceData {
    /// The currency of the balance.
    pub currency: String,
    /// The available balance.
    pub available_balance: f64,
    /// The ledger balance.
    pub ledger_balance: f64,
}

// ---------------------- Swap currencies ----------------------
/// Represents the options for swapping currencies.
#[derive(Debug, Clone, Serialize)]
pub struct SwapOptions {
    /// The amount to be swapped.
    pub amount: f64,
    /// The currency to swap from.
    pub from: String,
    /// The currency to swap to.
    pub to: String,
}

/// Represents the data received after performing a currency swap.
#[derive(Debug, Deserialize)]
pub struct SwapData {
    /// The status of the swap.
    pub status: String,
    /// The reference ID of the swap.
    pub ref_id: String,
    /// The currency being swapped from.
    pub from_currency: String,
    /// The currency being swapped to.
    pub to_currency: String,
    /// The amount being swapped.
    pub amount: f64,
    /// The exchanged amount after the swap.
    pub exchanged_amount: f64,
    /// The charge applied to the swap.
    pub charge: f64,
    /// The exchange rate used for the swap.
    pub rate: f64,
    /// The creation timestamp of the swap.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the swap.
    pub updated_at: DateTime<Utc>,
}
