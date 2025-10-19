use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};
#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config);

    let tx_ref = String::from("mail_order_injera");
    let verification_result = client.verify_transaction(&tx_ref).await;
    match verification_result {
        Ok(verification_data) => println!("{:#?}", verification_data),
        Err(e) => eprintln!("{:#?}", e),
    }
}
