//! Models related to bank transfers.

use serde::{Deserialize, Serialize};

/// Represents the options required to initiate a bank transfer.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOptions {
    /// The name of the account holder.
    pub account_name: String,
    /// The bank account number to which the transfer will be made.
    pub account_number: String,
    /// The amount to be transferred.
    pub amount: String,
    /// The currency in which the transfer will be made.
    pub currency: String,
    /// A unique reference for the transfer.
    pub reference: String,
    /// The bank code of the recipient's bank.
    pub bank_code: u32,
}

/// Represents the response received after initiating a bank transfer.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    /// A message providing additional information about the transfer.
    pub message: String,
    /// The status of the transfer (e.g., "pending", "completed").
    pub status: String,
    /// Additional data related to the transfer.
    pub data: String,
}
