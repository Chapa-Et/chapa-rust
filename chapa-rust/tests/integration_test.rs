use chapa_rust::get_banks;
use chapa_rust::transaction::{verify_transaction, TransactionBuilder};
use chapa_rust::types::{Currency, TransactionResponse};

#[tokio::test]
async fn initialize_transaction() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let tx_ref = "test_reference123";

    let builder = TransactionBuilder::new(100);
    let transaction = builder
        .currency(Currency::ETB)
        .first_name("Abreham Kassa")
        .tx_ref(tx_ref)
        .finish();

    let response = transaction.initiate().await?;
    match response {
        TransactionResponse::Success(message) => {
            assert_eq!(message.status, String::from("success"));
        }
        _ => (),
    }

    let _response = verify_transaction(tx_ref).await?;
    let _response = get_banks().await?;

    Ok(())
}
