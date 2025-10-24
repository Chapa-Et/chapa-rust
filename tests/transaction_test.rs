use chapa_rust::{
    client::ChapaClient, config::ChapaConfigBuilder, models::payment::InitializeOptions,
};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_initialize_transaction() {
    let mut server = mockito::Server::new_async().await;
    let success = server
            .mock("POST", "/v1/transaction/initialize")
            .match_header(
                "authorization",
                Matcher::Regex(r#"^Bearer .+$"#.to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                serde_json::to_string(&serde_json::json!({
                "message": "Hosted Link",
                "status": "success",
                "data": {
                    "checkout_url": "https://checkout.chapa.co/checkout/payment/V38JyhpTygC9QimkJrdful9oEjih0heIv53eJ1MsJS6xG"
                    }
                }))
                .unwrap(),
            )
            .create_async()
            .await;

    let failure = server
        .mock("POST", "/v1/transaction/initialize")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Authorization required	",
              "status": "failed",
              "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK-xxxxxxxxxxxxxxxx")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let transaction_success = InitializeOptions {
        amount: "100".to_string(),
        currency: "ETB".to_string(),
        email: Some("customer@gmail.com".to_string()),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        tx_ref: String::from("some_generated_tax_ref"),
        ..Default::default()
    };
    let transaction_failure = InitializeOptions {
        ..Default::default()
    };

    // ACT for success
    let response_success = client
        .initialize_transaction(transaction_success)
        .await
        .unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client
        .initialize_transaction(transaction_failure)
        .await
        .unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_verify_transaction() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transaction/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Payment details",
            "status": "success",
            "data": {
                "first_name": "Bilen",
                "last_name": "Gizachew",
                "email": "abebech_bekele@gmail.com",
                "currency": "ETB",
                "amount": 100,
                "charge": 3.5,
                "mode": "test",
                "method": "test",
                "type": "API",
                "status": "success",
                "reference": "6jnheVKQEmy",
                "tx_ref": "chewatatest-6669",
                "customization": {
                    "title": "Payment for my favourite merchant",
                    "description": "I love online payments",
                    "logo": null
                },
                "meta": null,
                "created_at": "2023-02-02T07:05:23.000000Z",
                "updated_at": "2023-02-02T07:05:23.000000Z"
              }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/transaction/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Invalid transaction or Transaction not found	",
            "status": "failed",
            "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // ACT for success
    let response_success = client.verify_transaction("chewatatest-6669").await.unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.verify_transaction("chewatatest-6669").await.unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_transactions() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transactions")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transactions retrieved successfully",
                "status": "success",
                "data": {
                    "transactions": [
                        {
                            "status": "pending",
                            "ref_id": "VcEu3Hf55JU",
                            "type": "Payment Link",
                            "created_at": "2024-07-27T02:22:46.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1301688,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "R6XqfcNVQjW",
                            "type": "Payment Link",
                            "created_at": "2024-06-30T04:31:46.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1145318,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "VcLgFpZscvV",
                            "type": "Payment Link",
                            "created_at": "2024-07-08T10:14:47.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1193072,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "Ayt3PirXMN1",
                            "type": "Payment Link",
                            "created_at": "2024-08-03T10:19:13.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1314626,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "o6IuCbhiGIF",
                            "type": "Payment Link",
                            "created_at": "2024-05-25T19:20:08.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1021442,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "Guv3OEqEtHH",
                            "type": "Payment Link",
                            "created_at": "2024-07-27T10:41:23.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1302068,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "Pe8uKQPOjoI",
                            "type": "Payment Link",
                            "created_at": "2024-04-04T12:22:32.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 899805,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "zNhRfpHQkq0",
                            "type": "Payment Link",
                            "created_at": "2024-04-04T12:22:42.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 899806,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "xsHpcWnt6dR",
                            "type": "Payment Link",
                            "created_at": "2024-06-18T22:37:55.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 1097431,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        },
                        {
                            "status": "pending",
                            "ref_id": "q3fdyKgQfjR",
                            "type": "Payment Link",
                            "created_at": "2024-03-29T01:57:52.000000Z",
                            "currency": "ETB",
                            "amount": "12.000",
                            "charge": "0.000",
                            "trans_id": null,
                            "payment_method": "card",
                            "customer": {
                                "id": 883206,
                                "email": null,
                                "first_name": null,
                                "last_name": null,
                                "mobile": null
                            }
                        }
                    ],
                    "pagination": {
                        "per_page": 10,
                        "current_page": 1,
                        "first_page_url": "https://api.chapa.co/v1/transactions?page=1",
                        "next_page_url": "https://api.chapa.co/v1/transactions?page=2",
                        "prev_page_url": null
                    }
                }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/transactions")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Invalid API Key or  the business can't accept payments at the moment. Please verify your API key and ensure the account is active and able to process payments.",
                "status": "failed",
                "data": null
            })).unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // ACT for success
    let response_success = client.get_transactions().await.unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.get_transactions().await.unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_logs() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transaction/events/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transaction events fetched",
                "status": "success",
                "data": [
                    {
                        "item": 23445,
                        "message": "Attempted to make payment with telebirr USSD",
                        "type": "log",
                        "created_at": "2024-07-23T07:31:32.000000Z",
                        "updated_at": "2024-07-23T07:31:32.000000Z"
                    },
                    {
                        "item": 23567,
                        "message": "Transaction is successful with TELEBIRR - RSLT",
                        "type": "log",
                        "created_at": "2024-07-23T07:31:55.000000Z",
                        "updated_at": "2024-07-23T07:31:55.000000Z"
                    },
                    {
                        "item": 24678,
                        "message": "Redirecting to confirmation page",
                        "type": "log",
                        "created_at": "2024-07-23T07:31:32.000000Z",
                        "updated_at": "2024-07-23T07:31:32.000000Z"
                    }
                ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/transaction/events/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transaction not found",
                "status": "failed",
                "data": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let config = ChapaConfigBuilder::new()
        .base_url(server.url())
        .api_key("CHASECK_TEST-XXXXXXXXXXXXXXX")
        .build()
        .unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // ACT for success
    let response_success = client
        .get_transaction_logs("chewatatest-6669")
        .await
        .unwrap();
    assert_eq!(response_success.status, "success");
    assert!(!response_success.message.is_null()); // NOTE: ckeck if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client
        .get_transaction_logs("chewatatest-6669")
        .await
        .unwrap();
    assert_eq!(response_failure.status, "failed");
    assert!(!response_failure.message.is_null()); // NOTE: check if it is empty because I suspect there might be a change if I put string comparison.
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}
