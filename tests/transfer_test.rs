use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::transfer::{BulkData, BulkTransferOptions, TransferOptions},
};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_initiate_transfers() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("POST", "/v1/transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Transfer Queued Successfully",
              "status": "success",
              "data": "3241342142sfdd"
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let failure = server
        .mock("POST", "/v1/transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Insufficient Balance",
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
    let transfer_payload = TransferOptions {
        account_name: Some(String::from("Israel Goytom")),
        account_number: "32423423".into(),
        amount: "1".into(),
        currency: Some(String::from("ETB")),
        reference: Some(String::from("3241342142sfdd")),
        bank_code: 656,
    };

    // ACT for success
    let response_success = client.transfer(transfer_payload.clone()).await.unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());
    assert_eq!(response_success.status, "success");

    // ACT for failure
    let response_failure = client.transfer(transfer_payload).await.unwrap();
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    assert_eq!(response_failure.status, "failed");

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_verify_transfers() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transfers/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Transfer details",
              "status": "success",
              "data": {
                  "account_name": "Israel Goytom",
                  "account_number": "21312331234123",
                  "mobile": null,
                  "currency": "ETB",
                  "amount": 100,
                  "charge": 0,
                  "mode": "live",
                  "transfer_method": "bank",
                  "narration": null,
                  "chapa_transfer_id": "4d6a7cb7-0d51-4c27-9a19-cc3f066c85a3",
                  "bank_code": 128,
                  "bank_name": "Bunna Bank",
                  "cross_party_reference": null,
                  "ip_address": "UNKNOWN",
                  "status": "success",
                  "tx_ref": "chewatatest-6669",
                  "created_at": "2022-07-26T16:38:32.000000Z",
                  "updated_at": "2023-01-10T07:09:08.000000Z"
              }
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let failure = server
        .mock("GET", "/v1/transfers/verify/chewatatest-6669")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Invalid transfer reference or Transfer is not found",
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
    let response_success = client.verify_transfer("chewatatest-6669").await.unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());
    assert_eq!(response_success.status, "success");

    // ACT for failure
    let response_failure = client.verify_transfer("chewatatest-6669").await.unwrap();
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    assert_eq!(response_failure.status, "failed");

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_bulk_transfer() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("POST", "/v1/bulk-transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
              "message": "Bulk transfer queued successfully",
              "status": "success",
              "data": {
                  "id": 4,
                  "created_at": "2024-03-20T08:56:24.000000Z"
              }
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let failure = server
        .mock("POST", "/v1/bulk-transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": {
                "bulk_data.1.amount": [
            "The amount field is required"
                ]
            },
            "status": "failed"
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
    let bulk_transfer_payload = BulkTransferOptions {
        title: String::from("This Month Salary!"),
        currency: String::from("ETB"),
        bulk_data: vec![
            BulkData {
                account_name: Some(String::from("Israel Goytom")),
                account_number: String::from("09xxxxxxxx"),
                amount: String::from("1"),
                reference: Some(String::from("b1111124")),
                bank_code: 128,
            },
            BulkData {
                account_name: Some(String::from("Israel Goytom")),
                account_number: String::from("09xxxxxxxx"),
                amount: String::from("1"),
                reference: Some(String::from("b2222e5r")),
                bank_code: 128,
            },
        ],
    };
    // ACT for success
    let response_success = client
        .bulk_transfer(bulk_transfer_payload.clone())
        .await
        .unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());
    assert_eq!(response_success.status, "success");

    // ACT for failure
    let response_failure = client.bulk_transfer(bulk_transfer_payload).await.unwrap();
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    assert_eq!(response_failure.status, "failed");

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_verify_bulk_transfer() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transfers?batch_id=1")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transfer details fetched",
                "status": "success",
                "meta": {
                    "current_page": 1,
                    "first_page_url": "https://api.chapa.co/v1/transfers?page=1",
                    "last_page": 1,
                    "last_page_url": "https://api.chapa.co/v1/transfers?page=1",
                    "next_page_url": null,
                    "path": "https://api.chapa.co/v1/transfers?page=1",
                    "per_page": 10,
                    "prev_page_url": null,
                    "to": 2,
                    "total": 2,
                    "error": []
                },
                "data": [
                    {
                        "account_name": "Israel Goytom",
                        "account_number": null,
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "wallet",
                        "chapa_reference": "smtlsmH436t6",
                        "bank_code": 128,
                        "bank_name": "telebirr",
                        "bank_reference": "BCJ8FVX8AG",
                        "status": "success",
                        "reference": "b2222e5r",
                        "created_at": "2024-03-19T20:05:45.000000Z",
                        "updated_at": "2024-03-19T20:06:10.000000Z"
                    },
                    {
                        "account_name": "Israel Goytom",
                        "account_number": null,
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "wallet",
                        "chapa_reference": "VjYYS6TguXaL",
                        "bank_code": 128,
                        "bank_name": "telebirr",
                        "bank_reference": "BCJ0FVX87Q",
                        "status": "success",
                        "reference": "b1111124",
                        "created_at": "2024-03-19T20:05:45.000000Z",
                        "updated_at": "2024-03-19T20:06:06.000000Z"
                    }
                ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let failure = server
        .mock("GET", "/v1/transfers?batch_id=1")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "status": "failed",
                "message": "The Endpoint you are looking for is not found. Please refer to our documentation for more. developer.chapa.co"
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
    let response_success = client.verify_bulk_transfer("1").await.unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());
    assert!(response_success.meta.is_some());
    assert_eq!(response_success.status, "success");

    // ACT for failure
    let response_failure = client.verify_bulk_transfer("1").await.unwrap();
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    assert_eq!(response_failure.status, "failed");

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_get_transfers() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("GET", "/v1/transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Transfer details fetched",
                "status": "success",
                "meta": {
                    "current_page": 1,
                    "first_page_url": "https://api.chapa.co/v1/transfers?page=1",
                    "last_page": 16,
                    "last_page_url": "https://api.chapa.co/v1/transfers?page=16",
                    "next_page_url": "https://api.chapa.co/v1/transfers?page=2",
                    "path": "https://api.chapa.co/v1/transfers?page=1",
                    "per_page": 10,
                    "prev_page_url": null,
                    "to": 10,
                    "total": 159,
                    "error": []
                },
                "data": [
                    {
                        "account_name": "suz",
                        "account_number": "1",
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636706566",
                        "bank_code": 656,
                        "bank_name": "Awash Bank",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": null,
                        "created_at": "2022-10-24T14:46:56.000000Z",
                        "updated_at": "2023-08-07T10:49:59.000000Z"
                    },
                    {
                        "account_name": "suz",
                        "account_number": "1",
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636674388",
                        "bank_code": 656,
                        "bank_name": "Awash Bank",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": "7039636674388",
                        "created_at": "2022-10-24T14:48:06.000000Z",
                        "updated_at": "2023-10-10T09:20:27.000000Z"
                    },
                    {
                        "account_name": "Kidus Yared",
                        "account_number": "1000180831502",
                        "currency": "ETB",
                        "amount": 10,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "703963635595",
                        "bank_code": 946,
                        "bank_name": "Commercial Bank of Ethiopia (CBE)",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": "703963635595",
                        "created_at": "2023-05-02T07:58:26.000000Z",
                        "updated_at": "2023-10-10T09:20:45.000000Z"
                    },
                    {
                        "account_name": "suz",
                        "account_number": "1",
                        "currency": "ETB",
                        "amount": 1,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636518995",
                        "bank_code": 347,
                        "bank_name": "Bank of Abyssinia",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": null,
                        "created_at": "2022-10-24T14:50:13.000000Z",
                        "updated_at": "2023-08-07T10:51:06.000000Z"
                    },
                    {
                        "account_name": "Tamiru",
                        "account_number": "1000089352731",
                        "currency": "ETB",
                        "amount": 10,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "0DKKHEVdZVj",
                        "bank_code": 893,
                        "bank_name": "Cooperative Bank of Oromia (COOP)",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": "0DKKHEVdZVj",
                        "created_at": "2023-05-24T13:31:16.000000Z",
                        "updated_at": "2023-10-10T09:20:55.000000Z"
                    },
                    {
                        "account_name": "Tamiru",
                        "account_number": "1000089352731",
                        "currency": "ETB",
                        "amount": 5,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "r4HNsWByn9G",
                        "bank_code": 893,
                        "bank_name": "Cooperative Bank of Oromia (COOP)",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": null,
                        "created_at": "2023-05-24T15:20:51.000000Z",
                        "updated_at": "2023-08-07T10:52:30.000000Z"
                    },
                    {
                        "account_name": "Kidus Yared",
                        "account_number": "1000180831502",
                        "currency": "ETB",
                        "amount": 5,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "7039636781315",
                        "bank_code": 946,
                        "bank_name": "Commercial Bank of Ethiopia (CBE)",
                        "bank_reference": "FT23240M6PV5",
                        "status": "success",
                        "reference": null,
                        "created_at": "2023-08-27T16:48:55.000000Z",
                        "updated_at": "2023-08-27T16:56:04.000000Z"
                    },
                    {
                        "account_name": "Tamiru",
                        "account_number": "1000089352731",
                        "currency": "ETB",
                        "amount": 3,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "hIKAHfvATqp",
                        "bank_code": 893,
                        "bank_name": "Cooperative Bank of Oromia (COOP)",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": null,
                        "created_at": "2023-05-24T15:47:57.000000Z",
                        "updated_at": "2023-08-07T10:53:01.000000Z"
                    },
                    {
                        "account_name": "Tamiru",
                        "account_number": "1000089352731",
                        "currency": "ETB",
                        "amount": 5,
                        "charge": 0,
                        "transfer_type": "bank",
                        "chapa_reference": "FFY5w8lEYRU",
                        "bank_code": 893,
                        "bank_name": "Cooperative Bank of Oromia (COOP)",
                        "bank_reference": null,
                        "status": "failed/cancelled",
                        "reference": "FFY5w8lEYRU",
                        "created_at": "2023-05-24T13:48:42.000000Z",
                        "updated_at": "2023-10-10T09:21:27.000000Z"
                    }
                ]
            }))
            .unwrap(),
        )
        .create_async()
        .await;
    let failure = server
        .mock("GET", "/v1/transfers")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Invalid API Key or User doesn't exist",
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
    let response_success = client.get_transfers().await.unwrap();
    assert!(!response_success.message.is_null());
    assert!(response_success.data.is_some());
    assert!(response_success.meta.is_some());
    assert_eq!(response_success.status, "success");

    // ACT for failure
    let response_failure = client.get_transfers().await.unwrap();
    assert!(!response_failure.message.is_null());
    assert!(response_failure.data.is_none());
    assert_eq!(response_failure.status, "failed");

    success.assert_async().await;
    failure.assert_async().await;
}
