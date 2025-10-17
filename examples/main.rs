use chapa_rust::models::ChapaStructs::Transaction;

fn main() {
    let _ = chapa_rust::get_banks();

    let test_transaction = Transaction {
        amount: 150,
        currency: String::from("USD"),
        email: String::from("john_doe@gmail.com"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        tx_ref: String::from("mail_order_injera"),
    };

    let _ = chapa_rust::initialize_transaction(test_transaction);

    let _ = chapa_rust::verify_transaction(String::from("mail_order_injera"));
}
