//! Response model for chapa API

use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    bank::{BalanceData, Bank, SwapData},
    payment::{CheckoutURL, GetTransactionsData, TransactionLog, VerifyPaymentData},
    transfer::{BulkTransferData, TransferMeta, TransfersData, VerifyTransferData},
};

/// Represents a generic response from the Chapa API.
#[derive(Debug, Clone, Deserialize)]
pub struct ChapaResponse<D> {
    /// The status message of the response.
    pub message: Value, // NOTE: Changed to Value to handle empty strings or other types, since some responses might return non-string messages
    #[serde(default = "unspecified_status")]
    /// The status of the response.
    pub status: String,
    #[serde(default)]
    /// The data section of the response.
    pub data: D,
}

/// Represents a generic response from the Chapa API with metadata.
/// This struct has one additional field called `meta` compared to [`ChapaResponse`].
#[derive(Debug, Clone, Deserialize)]
pub struct ChapaResponseWithMeta<D, M> {
    /// The status message of the response.
    pub message: Value, // NOTE: Changed to Value to handle empty strings or other types, since some responses might return non-string messages
    #[serde(default = "unspecified_status")]
    /// The status of the response.
    pub status: String,
    #[serde(default)]
    /// The data section of the response.
    pub data: D,
    #[serde(default)]
    /// The metadata associated with the response.
    pub meta: M,
}

fn unspecified_status() -> String {
    "Unspecified".to_string()
}

/// Type alias for GetBanksResponse, which contains a list of banks.
pub type GetBanksResponse = ChapaResponse<Option<Vec<Bank>>>;
/// Type alias for GetBalancesResponse, which contains balance data.
pub type GetBalancesResponse = ChapaResponse<Option<Vec<BalanceData>>>;
/// Type alias for SwapResponse, which contains swap data.
pub type SwapResponse = ChapaResponse<Option<SwapData>>;

//
// ------------------------------------- Transaction Responses ---------------------------------------------
//

/// Type alias for InitializeResponse, which contains the checkout URL.
pub type InitializeResponse = ChapaResponse<Option<CheckoutURL>>;
/// Type alias for VerifyResponse, which contains the verification data.
pub type VerifyResponse = ChapaResponse<Option<VerifyPaymentData>>;
/// Type alias for TransactionLogsResponse, which contains a list of transaction logs.
pub type TransactionLogsResponse = ChapaResponse<Option<Vec<TransactionLog>>>;
/// Type alias for GetTransactionsResponse, which contains a list of transactions along with pagination data.
pub type GetTransactionsResponse = ChapaResponse<Option<GetTransactionsData>>;

//
// ------------------------------------- Transfer Responses ---------------------------------------------
//

/// Type alias for Transfer response, which may contain an optional string (e.g., transfer ID).
pub type TransferResponse = ChapaResponse<Option<String>>;
/// Type alias for VerifyTransfer response, which contains the verification data for a transfer.
pub type VerifyTransferResponse = ChapaResponse<Option<VerifyTransferData>>;
/// Type alias for BulkTransfer response, which contains bulk transfer data.
pub type BulkTransferResponse = ChapaResponse<Option<BulkTransferData>>;
/// Type alias for GetTransfersResponse, which contains a list of transfers along with pagination metadata.
pub type GetTransfersResponse =
    ChapaResponseWithMeta<Option<Vec<TransfersData>>, Option<TransferMeta>>;
/// Type alias for VerifyBulkTransferResponse, which contains bulk transfer verification data along with metadata.
pub type VerifyBulkTransferResponse =
    ChapaResponseWithMeta<Option<Vec<TransfersData>>, Option<TransferMeta>>;
