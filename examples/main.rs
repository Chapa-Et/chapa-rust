use chapa_rust::{
    client::ChapaClient, config::ChapaConfigBuilder, models::payment::InitializeOptions,
};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // First initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config);

    // call the get_banks method
    let result = client.get_banks().await;
    match result {
        Ok(banks) => println!("{:#?}", banks),
        Err(e) => eprintln!("{:#?}", e),
    }

    let tx_ref = String::from("mail_order_injer");
    let test_transaction = InitializeOptions {
        amount: "150".to_string(),
        currency: String::from("USD"),
        email: Some(String::from("john_doe@gmail.com")),
        first_name: Some(String::from("John")),
        last_name: Some(String::from("Doe")),
        tx_ref: tx_ref.clone(),
        ..Default::default()
    };

    let init_success = client.initialize_transaction(test_transaction).await;
    match init_success {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("{:#?}", e),
    }

    let verification_result = client.verify_transaction(&tx_ref).await;
    match verification_result {
        Ok(verification_data) => println!("{:#?}", verification_data),
        Err(e) => eprintln!("{:#?}", e),
    }
}
