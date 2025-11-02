//! Direct charge modules for Chapa API interactions.
use serde::{Deserialize, Serialize};

/// Represents the supported direct charge types.
#[derive(Clone, Debug, Serialize)]
pub enum DirectChargeType {
    /// Telebirr payment type
    #[serde(rename = "telebirr")]
    Telebirr,
    /// M-Pesa payment type
    #[serde(rename = "mpesa")]
    Mpesa, // TODO: Check if it is mpesa or M-pesa
    /// Amole payment type
    #[serde(rename = "amole")]
    Amole,
    /// CBE Birr payment type
    #[serde(rename = "cbebirr")]
    CBEBirr,
    /// Coopay eBirr payment type
    #[serde(rename = "ebirr")]
    CoopayEbirr,
    /// Awash Birr payment type
    #[serde(rename = "awashbirr")]
    AwashBirr,
    #[serde(untagged)]
    /// Other supported payment types
    Other(String),
}
// -------------------------------------- Direct Charge -----------------------------------------------
/// Represents the options required to initiate a direct charge.
#[derive(Clone, Debug, Default, Serialize)]
pub struct DirectChargeOptions {
    /// The first name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The last name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The email of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The mobile number of the customer.
    pub mobile: String,

    /// The currency for the transaction.
    pub currency: String,

    /// The amount to be charged.
    pub amount: String,

    /// The unique transaction reference.
    pub tx_ref: String,
}

/// Represents the data received from a direct charge initiation.
#[derive(Debug, Deserialize)]
pub struct DirectChargeData {
    /// The authorization type for the direct charge.
    pub auth_type: String,
    /// The request ID for the direct charge.
    #[serde(rename = "requestID")]
    pub request_id: String,
    /// The metadata associated with the direct charge.
    pub meta: DirectChargeMeta,
    /// The mode of the direct charge.
    pub mode: String,
}

/// Represents the metadata associated with a direct charge.
#[derive(Debug, Deserialize)]
pub struct DirectChargeMeta {
    /// The message associated with the direct charge.
    pub message: String,
    /// The status of the direct charge.
    pub status: String,
    /// The reference ID of the direct charge.
    pub ref_id: String,
    /// The payment status of the direct charge.
    pub payment_status: String,
}

// -------------------------------------- Direct Charge Validation(authorization) -----------------------------------------------

/// Represents the options required to verify a direct charge.
#[derive(Debug, Clone, Serialize)]
pub struct VerifyDirectChargeOption {
    /// The reference ID of the direct charge to be verified.
    pub reference: String,
    /// The client identifier for the verification request.
    pub client: String,
}
