<h1 align="center">
<div align="center">
  <a href="http://chapa.co/" target="_blank">
    <img src="https://assets.chapa.co/assets/images/chapa-logo.svg" width="320" alt="Chapa Logo"/>
  </a>
  <p align="center"> Unofficial Rust SDK for the [Chapa API](https://developer.chapa.co/docs/)</p>
</div>
</h1>



## Table of Contents
1. [Documentation](#documentation)
2. [Setup](#setup)
3. [Usage](#usage)
4. [Contribution](#contribution)
5. [Example](#example)
6. [License](#license)

## Setup
 The first step is to set up an account on [Chapa's](https://www.chapa.co) homepage. Once you're done, you should have public and secret API key available to you. Copy your secret key and paste it into a .env file in the same directory as your Rust project.
```js
    CHAPA_API_PUBLIC_KEY=<API_KEY>
```
> Note: Since this library has not yet been published on crates.io, you will not be able to simply declare this library as a dependency in your cargo.toml. Please clone the repo and import the library as a crate as follows. Apologies for the inconvenience.
```rs
    extern crate chapa_rust;
    use chapa_rust::Transaction;
```

## Usage
### 1. Get List of Banks   
Under the hood, this is a GET request that deserializes the response from the Chapa API into a struct named BankRequestResponse. You can find all structs in ChapaStructs.rs if you would like to implement new or custom functionality.

    
```rs
#[tokio::main]
pub async fn get_banks() -> Result<(), Box<dyn std::error::Error>> {
    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let banks_url = format!("{}/{}/banks", CHAPA_BASE_URL, version);
    let response = client.get(banks_url).headers(headers).send().await?;

    // Deserialization into Bank and BankRequestResponse structs
    let response_json = response.json::<BankRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}
```

You can expect a similar response in the following format to be printed to the terminal.

```rs
BankRequestResponse {
    message: "Banks retrieved",
    data: [
        Bank {
            id: "971bd28c-ff80-420b-a0db-0a1a4be6ee8b",
            swift: "ABAYETAA",
            name: "Abay Bank",
            acct_length: 16,
            country_id: 1,
            created_at: "2023-01-24T07:28:30.000000Z",
            updated_at: "2023-01-24T07:28:30.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "80a510ea-7497-4499-8b49-ac13a3ab7d07",
            swift: "AWINETAA",
            name: "Awash Bank",
            acct_length: 14,
            country_id: 1,
            created_at: "2022-03-17T07:21:30.000000Z",
            updated_at: "2022-07-05T00:34:03.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "32735b19-bb36-4cd7-b226-fb7451cd98f0",
            swift: "ABYSETAA",
            name: "Bank of Abyssinia",
            acct_length: 8,
            country_id: 1,
            created_at: "2022-07-05T00:33:57.000000Z",
            updated_at: "2022-07-05T00:33:57.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "96e41186-29ba-4e30-b013-2ca36d7e7025",
            swift: "CBETETAA",
            name: "Commercial Bank of Ethiopia (CBE)",
            acct_length: 13,
            country_id: 1,
            created_at: "2022-03-17T07:21:18.000000Z",
            updated_at: "2022-07-05T00:34:43.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "809814c1-ab98-4750-a5b8-3be5db7bd5f5",
            swift: "DASHETAA",
            name: "Dashen Bank",
            acct_length: 13,
            country_id: 1,
            created_at: "2022-11-15T06:17:43.000000Z",
            updated_at: "2022-11-15T06:17:43.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "800144e5-ae3d-4fc9-a25d-0632f31f5c73",
            swift: "UNTDETAA",
            name: "Hibret Bank",
            acct_length: 16,
            country_id: 1,
            created_at: "2023-01-06T06:18:43.000000Z",
            updated_at: "2023-01-06T06:18:43.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "853d0598-9c01-41ab-ac99-48eab4da1513",
            swift: "TELEBIRR",
            name: "telebirr",
            acct_length: 10,
            country_id: 1,
            created_at: "2022-12-12T17:41:12.000000Z",
            updated_at: "2022-12-12T17:41:12.000000Z",
            is_mobilemoney: Some(
                1,
            ),
        },
        Bank {
            id: "742a2912-01e5-4e04-baab-b2cc4fc20f8b",
            swift: "WEGAETAA",
            name: "Wegagen Bank",
            acct_length: 13,
            country_id: 1,
            created_at: "2022-11-15T06:16:40.000000Z",
            updated_at: "2022-11-15T06:17:31.000000Z",
            is_mobilemoney: None,
        },
        Bank {
            id: "32b1c5b7-1ca3-4da0-aedf-3c0aaac5277e",
            swift: "ZEMEETAA",
            name: "Zemen Bank",
            acct_length: 16,
            country_id: 1,
            created_at: "2022-09-30T15:56:53.000000Z",
            updated_at: "2022-09-30T15:56:53.000000Z",
            is_mobilemoney: None,
        },
    ],
}
```

### 2. Initialize Transaction  
Initilzing a transaction does require an input argument formatted according to the `Transaction` struct you imported during setup:
```rs
pub struct Transaction {
    pub amount: u32,
    pub currency: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub tx_ref: String
}
```
Here's a simple example:
```rs
    let test_transaction = Transaction {
        amount: 150,
        currency: String::from("USD"),
        email: String::from("john_doe@gmail.com"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        tx_ref: String::from("mail_order_injera"),
    };
```

Under the hood, this is a POST request to the Chapa API. It also does the work of serializing the Transaction struct and deserializing the response to a InitializeRequestResponse object for you.

```rs
#[tokio::main]
pub async fn initialize_transaction(
    transaction: Transaction,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Transaction");
    println!("{}", transaction.currency);

    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let init_url = format!("{}/{}/transaction/initialize", CHAPA_BASE_URL, version);

    let response = client
        .post(init_url)
        .headers(headers)
        .form(&transaction)
        .send()
        .await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<InitializeRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}
```

You can expect a similar response in the following format to be printed to the terminal.

```
Initializing Transaction
USD
InitializeRequestResponse {
    message: "Hosted Link",
    status: "success",
    data: Some(
        CheckoutURL {
            checkout_url: "https://checkout.chapa.co/checkout/payment/kZrB1LIPpYGhd0Q9mIB7BHCDnDRrAUIgehKMasnJzT0zm",
        },
    ),
}
```
### 3. Verify Transaction  
Verifying a transaction does not involve any struct arguments, but does require a string that represents the tx_ref field of the previously initialized transaction. Similarly to initializing a transaction, this function does work of deserailzing the API's response into a VerifyRequestResponse struct for you:
```rs
#[tokio::main]
pub async fn verify_transaction(tx_ref: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Verifying Transaction");

    const CHAPA_BASE_URL: &str = "https://api.chapa.co";
    let version = "v1";
    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let verify_url = format!(
        "{}/{}/transaction/verify/{}",
        CHAPA_BASE_URL, version, tx_ref
    );

    println!("{}", verify_url);

    let response = client.get(verify_url).headers(headers).send().await?;

    // Deserialization into InitializeRequestResponse struct
    let response_json = response.json::<VerifyRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(())
}
```
You can expect a similar response in the following format to be printed to the terminal.

```rs
Verifying Transaction
https://api.chapa.co/v1/transaction/verify/test_transac_tx_ref
VerifyRequestResponse {
    message: "Payment details",
    status: "success",
    data: FullTransactionInfo {
        first_name: "Abebe",
        last_name: "Bikila",
        email: "abebe@bikila.com",
        currency: "USD",
        amount: 100,
        charge: 1,
        mode: "test",
        method: "test",
        type: "API",
        status: "success",
        reference: "66446606376",
        tx_ref: "test_transac_tx_ref",
        customization: CustomizationInfo {
            title: None,
            description: None,
            logo: None,
        },
        meta: None,
        created_at: "2023-01-03T15:11:47.000000Z",
        updated_at: "2023-01-03T15:11:47.000000Z",
    },
}
```

## Example  
Here's a full example involving all necessary imports, a request to retrieve a list of banks, initialzing a transaction and verifying said transaction.

```rs 
extern crate chapa_rust;
use chapa_rust::Transaction;

fn main() {
    chapa_rust::get_banks();

    let my_transaction = Transaction {
        amount: 150,
        currency: String::from("USD"),
        email: String::from("john_doe@gmail.com"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        tx_ref: String::from("mail_order_injera"),
    };

    chapa_rust::initialize_transaction(my_transaction);
    chapa_rust::verify_transaction("mail_order_injera");
    
}
```



## Contribution
If you find a bug or have any suggestions, please feel free to open an issue or a pull request.

## License
This open source library is under the terms of the MIT license.





