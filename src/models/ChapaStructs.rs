use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct BankRequestResponse {
    message: String,
    data: Vec<Bank>,
}

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

#[derive(Debug, Deserialize)]
pub struct InitializeRequestResponse {
    message: String,
    status: String,
    data: Option<CheckoutURL>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutURL {
    checkout_url: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequestResponse {
    message: String,
    status: String,
    data: FullTransactionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: u32,
    pub currency: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub tx_ref: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomizationInfo {
    title: Option<String>,
    description: Option<String>,
    logo: Option<String>,
}
