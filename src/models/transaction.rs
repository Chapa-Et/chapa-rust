//! Models related to get_transactions API responses.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the response from Chapa when fetching all transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    /// The status message of the response.
    pub message: String,
    /// The status of the response.
    pub status: String,
    /// The data containing the list of transactions and pagination info.
    pub data: GetTransactionsData,
}

/// Represents the data section of the GetTransactionsResponse.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsData {
    /// The list of transactions.
    pub transactions: Vec<Transaction>,
    /// The pagination information.
    pub pagination: Pagination,
}

/// Represents a customer associated with a transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    /// The unique identifier of the customer.
    pub id: u32,
    /// The first name of the customer.
    pub first_name: String,
    /// The last name of the customer.
    pub last_name: String,
    /// The email address of the customer.
    pub email: String,
    /// The mobile number of the customer.
    pub mobile: String,
}

/// Represents a transaction in Chapa.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// The status of the transaction.
    pub status: String,
    /// The reference ID of the transaction.
    pub ref_id: String,
    /// The type of the transaction. eg. "Payment Link"
    pub r#type: String,
    /// The date and time when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The currency in which the transaction was made.
    pub currency: String,
    /// The amount of money that is involved in the transaction.
    pub amount: String,
    /// The charge applied to the transaction.
    pub charge: String,
    /// The unique identifier of the transaction.
    pub trans_id: String,
    /// The payment method used for the transaction.
    pub payment_method: String,
    /// The customer associated with the transaction.
    pub customer: Customer,
}

/// Represents pagination details for a list of transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    /// How many transactions are in a single page.
    pub per_page: u32,
    /// Page number of the current set of transactions.
    pub current_page: u32,
    /// URL to the first page of transactions.
    pub first_page_url: String,
    /// URL to the next page of transactions.
    pub next_page_url: Option<String>,
    /// URL to the previous page of transactions.
    pub prev_page_url: Option<String>,
}
