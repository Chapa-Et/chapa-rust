use crate::authorize;
use crate::types::{
    Currency, CustomizationInfo, InitializeRequestResponse, TransactionResponse,
    VerifyRequestResponse,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn verify_transaction(
    tx_ref: &str,
) -> Result<VerifyRequestResponse, Box<dyn std::error::Error>> {
    let chapa_base_url = env::var("CHAPA_BASE_URL")?;
    let version = env::var("CHAPA_VERSION")?;

    let headers = authorize()?; // NOTE: turbo-fished operation

    // Building client + making request
    let client = reqwest::Client::new();
    let verify_url = format!(
        "{}/{}/transaction/verify/{}",
        chapa_base_url, version, tx_ref
    );

    println!("{}", verify_url);

    let response = client.get(verify_url).headers(headers).send().await?;
    println!("{}", response.status().to_string());

    // Deserialization into VerifyRequestResponse struct
    let response_json = response.json::<VerifyRequestResponse>().await?;

    println!("{:#?}", response_json);

    Ok(response_json)
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Transaction {
    amount: u32,
    currency: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    tx_ref: Option<String>,
    phone_number: Option<String>,
    callback_url: Option<String>,
    return_url: Option<String>,
    customization: Option<CustomizationInfo>,
    meta: Option<bool>,
}

impl Transaction {
    pub async fn initiate(self: &Self) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
        let chapa_base_url = env::var("CHAPA_BASE_URL")?;
        let version = env::var("CHAPA_VERSION")?;

        let headers = authorize()?; // NOTE: turbo-fished operation

        // Building client + making request
        let client = reqwest::Client::new();
        let init_url = format!("{}/{}/transaction/initialize", chapa_base_url, version);

        let response = client
            .post(init_url)
            .headers(headers)
            .json(self)
            .send()
            .await?;
        let status = response.status();

        // Deserialization into InitializeRequestResponse struct
        let response_json = response.json::<InitializeRequestResponse>().await?;
        match status {
            StatusCode::OK => Ok(TransactionResponse::Success(response_json)),

            StatusCode::UNAUTHORIZED => {
                if response_json
                    .message
                    .to_lowercase()
                    .contains("authorization required")
                {
                    return Ok(TransactionResponse::AuthorizationRequired);
                }

                Ok(TransactionResponse::InvalidAPIKey)
            }

            StatusCode::BAD_REQUEST => {
                if response_json
                    .message
                    .to_lowercase()
                    .contains("invalid currency")
                {
                    return Ok(TransactionResponse::InvalidCurrency);
                } else if response_json
                    .message
                    .to_lowercase()
                    .contains("the subaccount id you provided isn't associated with this account")
                {
                    return Ok(TransactionResponse::InvalidSubaccountID);
                } else if response_json
                    .message
                    .to_lowercase()
                    .contains("merchant's share of payment is not enough to cover transaction fee")
                {
                    return Ok(TransactionResponse::InsufficientMerchantShare);
                } else if response_json
                    .message
                    .to_lowercase()
                    .contains("merchant fee is greater than split flat amount")
                {
                    return Ok(TransactionResponse::MerchantFeeExceedsSplitFlatAmount);
                } else if response_json
                    .message
                    .to_lowercase()
                    .contains("transaction reference has been used before")
                {
                    return Ok(TransactionResponse::DuplicateTransactionReference);
                } else if response_json
                    .message
                    .to_lowercase()
                    .contains("user can't receive payments")
                {
                    return Ok(TransactionResponse::UserCannotReceivePayment);
                }

                Ok(TransactionResponse::UnexpectedResponse(
                    response_json.message.clone(),
                ))
            }

            StatusCode::NOT_FOUND => Ok(TransactionResponse::ApiPaymentDisabled),
            _ => Ok(TransactionResponse::UnexpectedResponse(
                response_json.message.clone(),
            )),
        }
    }
}

pub struct TransactionBuilder {
    tx: Transaction,
}

impl TransactionBuilder {
    pub fn new(amount: u32) -> Self {
        TransactionBuilder {
            tx: Transaction {
                amount,
                ..Default::default()
            },
        }
    }

    pub fn currency(mut self: Self, cur: Currency) -> Self {
        match cur {
            Currency::ETB => {
                self.tx.currency = Some(String::from("ETB"));
            }
            Currency::USD => {
                self.tx.currency = Some(String::from("USD"));
            }
        }

        return self;
    }

    pub fn email(mut self: Self, email: &str) -> Self {
        self.tx.email = Some(email.to_owned());
        self
    }

    pub fn first_name(mut self: Self, f_name: &str) -> Self {
        self.tx.first_name = Some(f_name.to_owned());
        self
    }

    pub fn last_name(mut self: Self, l_name: &str) -> Self {
        self.tx.last_name = Some(l_name.to_owned());
        self
    }

    pub fn tx_ref(mut self: Self, reference: &str) -> Self {
        self.tx.tx_ref = Some(reference.to_owned());
        self
    }

    pub fn phone_number(mut self: Self, p_number: &str) -> Self {
        self.tx.phone_number = Some(p_number.to_owned());
        self
    }

    pub fn callback_url(mut self: Self, url: &str) -> Self {
        self.tx.callback_url = Some(url.to_owned());
        self
    }

    pub fn return_url(mut self: Self, url: &str) -> Self {
        self.tx.return_url = Some(url.to_owned());
        self
    }

    pub fn customization(mut self: Self, customization: CustomizationInfo) -> Self {
        self.tx.customization = Some(customization);
        self
    }

    pub fn meta(mut self: Self, m: bool) -> Self {
        self.tx.meta = Some(m);
        self
    }

    pub fn finish(self: Self) -> Transaction {
        self.tx
    }
}
