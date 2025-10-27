use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::payment::{Customization, InitializeOptions},
    models::bank::Currency
};
#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let tx_ref = String::from("mail_order_injera");
    let test_transaction = InitializeOptions {
        amount: "150".to_string(),
        currency: Currency::USD,
        email: Some(String::from("john_doe@gmail.com")),
        first_name: Some(String::from("John")),
        last_name: Some(String::from("Doe")),
        tx_ref: tx_ref.clone(),
        customization: Some(Customization {
            title: Some("Injera Purchase".to_string()), // CONSIDER: make the title very long to observe an error
            description: Some("Order 1234 - 5kg of Injera".to_string()), // CONSIDER: use "#" inside to observe an error
            logo: Some("https://example.com/logo.png".to_string()),
        }),
        ..Default::default()
    };

    let init_success = client.initialize_transaction(test_transaction).await;
    match init_success {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => eprintln!("{:#?}", e),
    }
}
