use chapa_rust::{client::ChapaClient, config::ChapaConfigBuilder, models::bank::SwapOptions};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let swap_options = SwapOptions {
        from: String::from("USD"),
        to: String::from("ETB"),
        amount: 10.0,
    };
    // call the swap_currencies method
    let result = client.swap_currencies(swap_options).await;
    match result {
        Ok(swap_response) => println!("{:#?}", swap_response),
        Err(e) => eprintln!("{:#?}", e),
    }
}
