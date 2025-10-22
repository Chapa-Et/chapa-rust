//! # Chapa Models
//! This module defines all data structures used for interacting with the
//! [Chapa API](https://developer.chapa.co/).  
//! It includes response and request types for banks, transactions, and verifications.
//! These models leverage [`serde`](https://docs.rs/serde) for easy JSON
//! serialization and deserialization.
//! ## Example
//! ```rust,no_run
//! use chapa_rust::models::payment::InitializeOptions;
//!
//! // Create a transaction
//! let tx = InitializeOptions {
//!     amount: "100".to_string(),
//!     currency: "ETB".to_string(),
//!     email: Some("user@example.com".to_string()),
//!     first_name: Some("John".to_string()),
//!     last_name: Some("Doe".to_string()),
//!     tx_ref: "unique_tx_1234".to_string(),
//!     ..Default::default()
//! };
//! ```
//!
//! All response models can be directly deserialized from Chapa API JSON responses.

pub mod bank;
pub mod payment;
pub mod response;
pub mod transaction;
pub mod transfer;
