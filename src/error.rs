//! Error handling module for Chapa API interactions.
use thiserror::Error;

/// A specialized `Result` type for Chapa operations.
pub type Result<T> = std::result::Result<T, ChapaError>;

/// Represents Application-level errors for Chapa API interactions.
#[derive(Error, Debug)]
pub enum ChapaError {
    /// Indicates that the API key is missing in the configuration.
    #[error(
        "API Key is required but not set. Please set it using the CHAPA_API_PUBLIC_KEY environment variable or via the builder's api_key() method."
    )]
    MissingApiKey,
    /// Indicates that a network error occurred.
    #[error("Network error occurred")]
    NetworkError(#[from] reqwest::Error),
    /// Invalid HTTP method
    #[error("Invalid HTTP method: {0}")]
    InvalidHttpMethod(String),
    /// Represents an error returned by the Chapa API.
    #[error("API error occurred: {0}")]
    ApiError(String),
    /// Indicates that a header value is invalid.
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(String),
    /// Indicates that a header name is invalid.
    #[error("Invalid header name: {0}")]
    InvalidHeaderName(String),
    /// Represents JSON serialization or deserialization errors.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
