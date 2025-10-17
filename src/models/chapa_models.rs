#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Represents the full response returned when fetching the list of supported banks.
#[derive(Debug, Deserialize)]
pub struct BankRequestResponse {
    message: String,
    data: Vec<Bank>,
}

/// Represents a single bank entry from Chapa’s bank list.
#[derive(Debug, Deserialize)]
pub struct Bank {
    id: String,
    swift: String,
    name: String,
    acct_length: u32,
    country_id: u32,
    created_at: String,
    updated_at: String,
    is_mobilemoney: Option<u32>,
}

/// Response from Chapa when initializing a transaction.
#[derive(Debug, Deserialize)]
pub struct InitializeRequestResponse {
    message: String,
    status: String,
    data: Option<CheckoutURL>,
}

/// Represents the checkout URL provided by Chapa after a successful initialization.
#[derive(Debug, Deserialize)]
pub struct CheckoutURL {
    checkout_url: String,
}

/// Response from verifying a transaction.
#[derive(Debug, Deserialize)]
pub struct VerifyRequestResponse {
    message: String,
    status: String,
    data: FullTransactionInfo,
}

/// Represents the structure of a transaction request sent to Chapa.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// The amount to be charged (in the smallest currency unit, e.g., cents or ETB fractional).
    pub amount: u32,
    /// The currency code (e.g. `"ETB"`, `"USD"`).
    pub currency: String,
    /// The customer’s email address.
    pub email: String,
    /// The customer’s first name.
    pub first_name: String,
    /// The customer’s last name.
    pub last_name: String,
    /// A unique transaction reference (used for verification).
    pub tx_ref: String,
}

/// Contains full transaction details returned after a verification request.
#[derive(Debug, Serialize, Deserialize)]
pub struct FullTransactionInfo {
    first_name: String,
    last_name: String,
    email: String,
    currency: String,
    amount: u32,
    charge: u32,
    mode: String,
    method: String,
    r#type: String,
    status: String,
    reference: String,
    tx_ref: String,
    customization: CustomizationInfo,
    meta: Option<String>,
    created_at: String,
    updated_at: String,
}

/// Represents the visual and descriptive customization options for a transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomizationInfo {
    title: Option<String>,
    description: Option<String>,
    logo: Option<String>,
}
