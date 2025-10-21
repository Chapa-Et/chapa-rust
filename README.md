<h1 align="center">
<div align="center">
  <a href="http://chapa.co/" target="_blank">
    <img src="https://assets.chapa.co/assets/images/chapa-logo.svg" width="320" alt="Chapa Logo"/>
  </a>
  <p align="center"> Unofficial Rust SDK for the <a href="https://developer.chapa.co/docs/" target="_blank">Chapa API</a></p>
</div>
</h1>

<!--[![Crates.io](https://img.shields.io/crates/v/chapa-rust.svg)](https://crates.io/crates/chapa-rust) -->
<!-- [![Docs.rs](https://docs.rs/chapa-rust/badge.svg)](https://docs.rs/chapa-rust) -->
[![Build Status](https://github.com/YabetsZ/chapa-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/YabetsZ/chapa-rust/actions)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

> A type-safe, async Rust SDK for the [Chapa Payment API](https://developer.chapa.co/).

>⚠️ **Pre-release Notice**
>
> `chapa-rust` is currently in active development (`v0.1.0`).
> While stable enough for testing and sandbox use, the public API may change before `v1.0.0`.


## Table of Contents
<!-- 1. [Documentation](#documentation) -->
1. [Setup](#setup)
2. [Usage](#usage)
3. [Contribution](#contribution)
4. [Example](#example)
5. [License](#license)

<!-- ## Documentation
Comprehensive documentation is available on [Docs.rs](https://docs.rs/chapa-rust) and includes:
- Detailed API reference for all modules, structs, and methods.
- Guides and examples for common use cases.
-->
## Setup

## Getting Started

`chapa-rust` is an **asynchronous, type-safe Rust SDK** for the [Chapa Payment API](https://developer.chapa.co/).  
It allows developers to easily integrate Chapa’s payment, transfer, and verification services in their Rust projects.

> ⚠️ **Note:** This is a pre-release version (`v0.1.0-alpha.1`) and is currently distributed directly via Git.  
> The API may still change slightly before the stable `v1.0.0` release.

---

## Installation

Add `chapa-rust` as a dependency in your project’s `Cargo.toml`.

### Option 1 — Use the main branch (latest stable development)
```toml
[dependencies]
chapa-rust = { git = "https://github.com/Chapa-Et/chapa-rust.git", branch = "main" }
```
### Option 2 — Use the development branch (latest experimental features)
```toml
[dependencies]
chapa-rust = { git = "https://github.com/Chapa-Et/chapa-rust.git", branch = "develop" }
```
Once added, run:
```bash
cargo build
```
## Configuration
 The first step is to set up an account on [Chapa's](https://www.chapa.co) homepage. Once you're done, you should have public and secret API keys available to you. Copy your secret key and paste it into a `.env` file in the same directory as your Rust project.
```bash
CHAPA_API_PUBLIC_KEY=<API_KEY>
```
> **Note:** The `CHAPA_API_PUBLIC_KEY` is required for the SDK to function.

## Usage
> You can refer to [the examples folder](https://github.com/Chapa-Et/chapa-rust/tree/main/examples) for a comprehensive treatment of each Chapa API operation.

> In this section, we will look at the transaction operation.


### Initialize Transaction  
Initializing a transaction requires an input argument formatted according to the `InitializeOption` struct you imported during setup:
```rs
pub struct InitializeOptions {
    /// The first name of the customer.
    pub first_name: Option<String>,
    /// The last name of the customer.
    pub last_name: Option<String>,
    /// The email address of the customer.
    pub email: Option<String>,
    /// The phone number of the customer.
    pub phone_number: Option<String>,
    /// The currency for the transaction (e.g., "ETB", "USD").
    pub currency: String,
    /// The amount to be charged in the transaction.
    pub amount: String,
    /// A unique reference for the transaction.
    pub tx_ref: String,
    /// An optional callback URL for transaction updates.
    pub callback_url: Option<String>,
    /// An optional return URL for redirecting after payment.
    pub return_url: Option<String>,
    /// Customization options for the payment interface.
    pub customization: Option<Customization>,
    /// Additional metadata to be associated with the transaction.
    pub meta: serde_json::Value,
}
```
> Notice the optional fields.
Here's a simple example:
```rs
    let test_transaction = InitializeOptions {
        amount: "150".to_string(),
        currency: String::from("USD"),
        email: Some(String::from("john_doe@gmail.com")),
        first_name: Some(String::from("John")),
        last_name: Some(String::from("Doe")),
        tx_ref: String::from("mail_order_injera"),
        ..Default::default()
    };
```

Under the hood, this is a POST request to the Chapa API. It also does the work of serializing the Transaction struct and deserializing the response to an `InitializeRequestResponse` object for you.

```rs
use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::payment::{Customization, InitializeOptions},
};
#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let tx_ref = String::from("mail_order_injera");
    let test_transaction = InitializeOptions {
        amount: "150".to_string(),
        currency: String::from("USD"),
        email: Some(String::from("john_doe@gmail.com")),
        first_name: Some(String::from("John")),
        last_name: Some(String::from("Doe")),
        tx_ref: tx_ref.clone(),
        customization: Some(Customization {
            title: Some("Injera Purchase".to_string()),
            description: Some("Order 1234 - 5kg of Injera".to_string()),
            logo: Some("https://example.com/logo.png".to_string()),
        }),
        ..Default::default()
    };

    let init_success = client.initialize_transaction(test_transaction).await;
    match init_success {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("{:#?}", e),
    }
}

```

You can expect a similar response in the following format to be printed to the terminal.

```json
{
   "message": "Hosted Link",
   "status": "success",
   "data": {
   "checkout_url": "https://checkout.chapa.co/checkout/payment/V38JyhpTygC9QimkJrdful9oEjih0heIv53eJ1MsJS6xG"
    }
}
```

> Using the `tx_ref` we used in the `InitializeOption` struct, you can request to verify the transaction using the `client.verify_transaction(tx_ref)` method. Refer to the [examples folder](https://github.com/Chapa-Et/chapa-rust/tree/main/examples/verify_transaction) for more.



## Contribution
If you find a bug or have any suggestions, please feel free to open an issue or a pull request.

## License
This open source library is under the terms of the MIT license.
