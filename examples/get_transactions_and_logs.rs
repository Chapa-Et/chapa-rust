use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // call the get_transactions method
    let result = client.get_transactions().await;
    match result {
        Ok(banks) => println!("{:#?}", banks),
        Err(e) => eprintln!("{:#?}", e),
    }
    // call the get_transaction_logs method
    let result_logs = client.get_transaction_logs("mail_order_injera").await;
    match result_logs {
        Ok(logs) => println!("{:#?}", logs),
        Err(e) => eprintln!("{:#?}", e),
    }
}
