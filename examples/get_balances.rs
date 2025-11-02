use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // call to get_balances method
    let result = client.get_balances().await;
    match result {
        Ok(balances) => println!("{:#?}", balances),
        Err(e) => eprintln!("{:#?}", e),
    }
    // OR call to get_balances_by_currency method for a specific currency
    let result_currency = client.get_balances_by_currency("ETB").await;
    match result_currency {
        Ok(balance) => println!("{:#?}", balance),
        Err(e) => eprintln!("{:#?}", e),
    }
}
