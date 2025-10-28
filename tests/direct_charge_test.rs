use chapa_rust::config::ChapaConfigBuilder;
use chapa_rust::models::direct_charge::{DirectChargeType, VerifyDirectChargeOption};
use chapa_rust::{client::ChapaClient, models::direct_charge::DirectChargeOptions};
use mockito::{self, Matcher};

#[tokio::test]
async fn test_direct_charge() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("POST", "/v1/charges?type=telebirr")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
                "message": "Charge initiated",
                "status": "success",
                "data": {
                    "auth_type": "ussd",
                    "requestID": "66dPW486w0z6uibrcraZ2diYztK2lx2WaslwGnS18UBXTctDxRdAudYtq3jJtMu7CV6gzyCpBSfrm9kKFJBsA8Wq7zKvk0UxL",
                    "meta": {
                        "message": "Payment successfully initiated with telebirr",
                        "status": "success",
                        "ref_id": "CH3mhMQVhsHm2",
                        "payment_status": "PENDING"
                    },
                    "mode": "live"
                }
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("POST", "/v1/charges?type=telebirr")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Authorization required",
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

    let charges_payload = DirectChargeOptions {
        amount: String::from("10"),
        currency: String::from("ETB"),
        tx_ref: String::from("12311se2319ud4"),
        mobile: String::from("09xxxxxxxx"),
        ..Default::default()
    };
    let charge_type = DirectChargeType::Telebirr;

    // ACT for success
    let response_success = client
        .direct_charge(&charge_type, charges_payload.clone())
        .await
        .unwrap();
    assert!(!response_success.message.is_null());
    assert_eq!(response_success.status, "success");
    assert!(response_success.data.is_some());

    // ACT for failure
    let response_failure = client
        .direct_charge(&charge_type, charges_payload)
        .await
        .unwrap();
    assert!(!response_failure.message.is_null());
    assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    success.assert_async().await;
    failure.assert_async().await;
}

#[tokio::test]
async fn test_verify_direct_charge() {
    let mut server = mockito::Server::new_async().await;
    let success = server
        .mock("POST", "/v1/validate?type=amole")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Payment is completed",
            "trx_ref": "CHS7WFpXdCMR0",
            "processor_id": null
            }))
            .unwrap(),
        )
        .create_async()
        .await;

    let failure = server
        .mock("POST", "/v1/validate?type=amole")
        .match_header(
            "authorization",
            Matcher::Regex(r#"^Bearer .+$"#.to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(&serde_json::json!({
            "message": "Invalid client data or Transaction is nowhere to be found.",
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

    let verify_charges_payload = VerifyDirectChargeOption {
        reference: String::from("CHcuKjgnN0Dk0"),
        client: String::from(
            "0jhd12Dfee+2h/FzHA/X1zPlDmRmH5v+F4sdsfFFSEgg44FAFDSFS000+YwUHegTSogQdnXp7OGdUxPngiv6592YoL0YXa4eHcH1fRGjAimdqucGJPurFVu4sE5gJIEmBCXdESVqNPG72PwdRPfAINT9x1bXemI1M3bBdydtWvAx58ZE4fcOtWkD/IDi+o8K7qpmzgUR8YUbgZ71yi0pg5UmrT4YpcY2eq5i46Gg3L+rtjhjkgjkjg83hfkjajhf3",
        ),
    };
    let charge_type = DirectChargeType::Amole;

    // ACT for success
    let response_success = client
        .verify_direct_charge(&charge_type, verify_charges_payload.clone())
        .await
        .unwrap();
    assert!(!response_success.message.is_empty());
    assert!(response_success.trx_ref.is_some());

    // ACT for failure
    let response_failure = client
        .verify_direct_charge(&charge_type, verify_charges_payload)
        .await
        .unwrap();
    assert!(!response_failure.message.is_empty());
    assert_eq!(response_failure.status, "failed");
    assert!(response_failure.data.is_none());

    // println!("Success Response: {:#?}", response_success);
    // println!("Failure Response: {:#?}", response_failure);

    success.assert_async().await;
    failure.assert_async().await;
}
