use chapa_rust::models::chapa_models::Transaction;

#[tokio::main]
async fn main() {
    let result = chapa_rust::get_banks().await;
    match result {
        Ok(banks) => println!("{:#?}", banks),
        Err(e) => eprintln!("{:#?}", e),
    }

    let test_transaction = Transaction {
        amount: 150,
        currency: String::from("USD"),
        email: String::from("john_doe@gmail.com"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        tx_ref: String::from("mail_order_injera"),
    };

    let init_success = chapa_rust::initialize_transaction(test_transaction)
        .await
        .inspect(|init_resp| println!("{:#?}", init_resp));

    if init_success.is_ok() {
        let verification_result = chapa_rust::verify_transaction(String::from("mail_order_injera"))
            .await
            .inspect(|ver| println!("{:#?}", ver));
        match verification_result {
            Ok(banks) => println!("{:#?}", banks),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
}
