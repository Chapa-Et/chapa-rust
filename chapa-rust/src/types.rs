use serde::{Deserialize, Serialize};

pub enum TransactionResponse {
    Success(InitializeRequestResponse),
    AuthorizationRequired,
    InvalidAPIKey,
    InvalidCurrency,
    InvalidSubaccountID,
    InsufficientMerchantShare,
    MerchantFeeExceedsSplitFlatAmount,
    DuplicateTransactionReference,
    UserCannotReceivePayment,
    ApiPaymentDisabled,
    UnexpectedResponse(String),
}

pub enum Currency {
    ETB,
    USD,
}

#[derive(Debug, Deserialize)]
pub struct InitializeRequestResponse {
    pub message: String,
    pub status: String,
    pub data: Option<CheckoutURL>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutURL {
    pub checkout_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomizationInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Bank {
    pub id: u32,
    pub swift: Option<String>,
    pub name: Option<String>,
    pub acct_length: Option<u32>,
    pub country_id: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_mobilemoney: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct BankRequestResponse {
    pub message: String,
    pub data: Vec<Bank>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FullTransactionInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub currency: Option<String>,
    pub amount: u32,
    pub charge: Option<f32>,
    pub mode: String,
    pub method: Option<String>,
    pub r#type: String,
    pub status: String,
    pub reference: Option<String>,
    pub tx_ref: Option<String>,
    pub customization: Option<CustomizationInfo>,
    pub meta: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequestResponse {
    pub message: String,
    pub status: String,
    pub data: Option<FullTransactionInfo>,
}
