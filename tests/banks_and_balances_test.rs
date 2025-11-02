use chapa_rust::config::ChapaConfigBuilder;
use chapa_rust::{client::ChapaClient, models::bank::SwapOptions};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_get_banks() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/banks")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Banks retrieved",
            "data": [
                {
                    "id": 130,
                    "slug": "abay_bank",
                    "swift": "ABAYETAA",
                    "name": "Abay Bank",
                    "acct_length": 16,
                    "country_id": 1,
                    "is_mobilemoney": null,
                    "is_active": 1,
                    "is_rtgs": 1,
                    "active": 1,
                    "is_24hrs": null,
                    "created_at": "2023-01-24T04:28:30.000000Z",
                    "updated_at": "2024-08-03T08:10:24.000000Z",
                    "currency": "ETB"
                }
            ]
                    }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/banks")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Invalid API Key	",
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

    // ACT for success
    let response_success = client.get_banks().await.unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.get_banks().await.unwrap();
    assert!(!response_failure.message.is_null());
    // assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_balances() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/balances")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "status": "success",
              "message": "Balance fetched successfully",
              "data": [
                {
                  "currency": "ETB",
                  "available_balance": 87416.03,
                  "ledger_balance": 0
                },
                {
                  "currency": "USD",
                  "available_balance": 5.97,
                  "ledger_balance": 0
                }
              ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/balances")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Invalid API Key",
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

    // ACT for success
    let response_success = client.get_balances().await.unwrap();
    assert!(!response_success.message.is_null());
    assert_eq!(response_success.status, "success");
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.get_balances().await.unwrap();
    assert!(!response_failure.message.is_null());
    assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_balances_by_currency() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/balances/ETB")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "status": "success",
              "message": "Balance fetched successfully",
              "data": [
                {
                  "currency": "ETB",
                  "available_balance": 87416.03,
                  "ledger_balance": 0
                }
              ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("GET", "/v1/balances/ETB")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Invalid API Key",
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

    // ACT for success
    let response_success = client.get_balances_by_currency("ETB").await.unwrap();
    assert!(!response_success.message.is_null());
    assert_eq!(response_success.status, "success");
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.get_balances_by_currency("ETB").await.unwrap();
    assert!(!response_failure.message.is_null());
    assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_swap_currencies() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("POST", "/v1/swap")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Swap has been made successfully.",
              "status": "success",
              "data": {
                "status": "Success",
                "ref_id": "SWPfSqc5BiwcC",
                "from_currency": "USD",
                "to_currency": "ETB",
                "amount": 1,
                "exchanged_amount": 127,
                "charge": 0,
                "rate": 127,
                "created_at": "2025-04-23T08:50:46.000000Z",
                "updated_at": "2025-04-23T08:50:46.000000Z"
              }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("POST", "/v1/swap")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Amount must be at least 1",
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

    let swap_payload = SwapOptions {
        amount: 100.0,
        from: "USD".to_string(),
        to: "ETB".to_string(),
    };

    // ACT for success
    let response_success = client.swap_currencies(swap_payload.clone()).await.unwrap();
    assert!(!response_success.message.is_null());
    assert_eq!(response_success.status, "success");
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client.swap_currencies(swap_payload).await.unwrap();
    assert!(!response_failure.message.is_null());
    assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}
