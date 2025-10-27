//! # Chapa Rust SDK
//!
//! `chapa-rust` is an **unofficial, community-maintained Rust SDK** for interacting with the
//! [Chapa Payments API](https://developer.chapa.co/).  
//! It enables developers to perform operations such as payment initialization, verification,
//! transfers, subaccount management, and more — with full async support built on top of
//! [Tokio](https://tokio.rs) and compatible with popular web frameworks like
//! [Axum](https://docs.rs/axum), [Actix-Web](https://actix.rs), and any Tower-based ecosystem.
//!
//! ---
//!
//! ## Features
//!
//! - Async-first design (using `reqwest` + `tokio`)
//! - Strongly-typed request/response models
//! - [TODO!] Full coverage of Chapa API endpoints
//! - [TODO!] Built-in validation and error handling
//! - [TODO!] Easy integration with web servers (Axum, Actix, Warp)
//! - [TODO!] Optional utilities (e.g. transaction reference generation)
//!
//! ---
//!
//! ## Core API Overview
//!
//! ```
//! use chapa_rust::client::ChapaClient;
//! use chapa_rust::models::payment::InitializeOptions;
//! use chapa_rust::models::bank::Currency;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = ChapaClient::new("YOUR_SECRET_KEY").unwrap();
//!
//!     let req = InitializeOptions {
//!         amount: "100".to_string(),
//!         currency: Currency::ETB,
//!         email: Some("customer@example.com".to_string()),
//!         first_name: Some("John".to_string()),
//!         last_name: Some("Doe".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let response = client.initialize_transaction(req).await.unwrap();
//!     println!("Payment status: {} \nPayment message: {}", response.status, response.message);
//! }
//! ```
//!
//! ---
//!
//! ## [TODO!] Supported Endpoints
//!
//! | Category | Methods |
//! |-----------|----------|
//! | Transactions | `initialize`, `verify`, `all_transactions`, `transaction_logs` |
//! | Split Payments | `split_payment` |
//! | Banks | `list_banks` |
//! | Subaccounts | `create_subaccount` |
//! | Transfers | `transfer`, `bulk_transfer`, `verify_transfer`, `all_transfers` |
//! | Direct Charges | `direct_charge`, `authorize_direct_charge` |
//! | Utilities | `generate_tx_ref()` |
//!
//! ---
//!
//! ## Error Handling
//!
//! Errors are represented by the [`ChapaError`](crate::error::ChapaError) enum,
//! which encapsulates HTTP, deserialization, and API-level errors.
//!
//!
//! ---
//!
//! ## Feature Flags [TODO!]
//!
//! - `blocking` — Enables blocking (non-async) client support  
//! - `serde` — Enables serialization and deserialization (enabled by default)  
//! - `logging` — Enables request/response logging (via `tracing` or `log`)  
//!
//! ```toml
//! [dependencies]
//! chapa-rust = { version = "0.1", features = ["serde", "logging"] }
//! ```
//!
//! ---
//!
//! ## Testing and Linting
//!
//! Run quality checks before committing:
//!
//! ```bash
//! make fmt
//! make lint
//! make test
//! ```
//!
//! All warnings are treated as errors via `cargo clippy -- -D warnings`.
//!
//! ---
//!
//! ## Contributing
//!
//! Contributions are welcome!  
//! See [`CONTRIBUTING.md`](https://github.com/Chapa-Et/chapa-rust/chapa-rust/blob/main/CONTRIBUTING.md)
//! for style conventions and documentation guidelines.
//!
//! ---
//!
//! ## License
//!
//! Licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
//! ---
//!
//! ## Notes
//!
//! This crate is **not officially affiliated** with Chapa.  
//! It aims to provide an ergonomic and type-safe developer experience for
//! Rust developers building payment systems in Ethiopia and beyond.
#![deny(missing_docs)]
pub mod client;
pub mod config;
pub mod error;
pub mod models;
