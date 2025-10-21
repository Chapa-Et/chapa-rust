//! Response model for chapa API

use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    bank::Bank,
    payment::{CheckoutURL, VerifyData},
};

/// Represents a generic response from the Chapa API.
#[derive(Debug, Clone, Deserialize)]
pub struct ChapaResponse<T> {
    /// The status message of the response.
    pub message: Value, // NOTE: Changed to Value to handle empty strings or other types, since some responses might return non-string messages
    #[serde(default = "unspecified_status")]
    /// The status of the response.
    pub status: String,
    /// The data section of the response.
    pub data: T,
}

fn unspecified_status() -> String {
    "Unspecified".to_string()
}

/// Type alias for GetBanksResponse, which contains a list of banks.
pub type GetBanksResponse = ChapaResponse<Option<Vec<Bank>>>;
/// Type alias for InitializeResponse, which contains the checkout URL.
pub type InitializeResponse = ChapaResponse<Option<CheckoutURL>>;
/// Type alias for VerifyResponse, which contains the verification data.
pub type VerifyResponse = ChapaResponse<Option<VerifyData>>;
