//! # Chapa Rust SDK
//!
//! - Setup: see [README.md](https://github.com/Chapa-Et/chapa-rust/blob/main/README.md)
//! - Examples: see the [`examples/`](https://github.com/Chapa-Et/chapa-rust/tree/main/examples) directory.
//! - Endpoints: see [ENDPOINTS.md](https://github.com/Chapa-Et/chapa-rust/blob/main/ENDPOINTS.md)
//! - Contributing: see [CONTRIBUTING.md](https://github.com/Chapa-Et/chapa-rust/blob/main/CONTRIBUTING.md)
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
//! - Built-in validation and error handling
//! - Easy integration with web servers (Axum, Actix, Warp)
//! - Optional utilities (e.g. transaction reference generation)
//!
//! ---
//!
//! ## Core API Overview
//!
//! ```
//! use chapa_rust::client::ChapaClient;
//! use chapa_rust::models::payment::InitializeOptions;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = ChapaClient::new("YOUR_SECRET_KEY").unwrap();
//!
//!     let req = InitializeOptions {
//!         amount: "100".to_string(),
//!         currency: "ETB".to_string(),
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
//! ## Supported Endpoints
//!
//! | Category | Methods |
//! |-----------|----------|
//! | Transactions | `initialize_transaction`, `verify_transaction`, `get_transactions`, `get_transaction_logs` |
//! | Banks | `list_banks`, `get_balances`, `get_balances_by_currency`, `swap_currencies`|
//! | Transfers | `transfer`, `bulk_transfer`, `verify_transfer`, `verify_bulk_transfer`, `get_transfers` |
//! | Direct Charges | `direct_charge`, `verify_direct_charge` |
//! | Utilities | `generate_tx_ref`, [TODO!] `encrypt_data` |
//! <!-- | Split Payments | `split_payment` | -->
//! <!-- | Subaccounts | `create_subaccount` | -->
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
//! ## Feature Flags
//!
//! - `blocking`[TODO!] — Enables blocking (non-async) client support  
//! - `logging` [TODO!] — Enables request/response logging (via `tracing` or `log`)
//! - `utils` — Enables utility functions (e.g., transaction reference generation)
//! - `encryption` — Enables data encryption utilities
//!
//! ```toml
//! [dependencies]
//! chapa-rust = { version = "0.1", features = ["encryption", "utils"] }
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
pub mod utils;
