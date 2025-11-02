use chapa_rust::{
    client::ChapaClient, config::ChapaConfigBuilder, models::transfer::TransferOptions,
};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    let transfer_payload = TransferOptions {
        account_name: Some(String::from("Yabets Zekaryas")),
        account_number: "096.......".into(), // account number of the account
        amount: "1".into(),
        currency: Some(String::from("ETB")),
        reference: Some(String::from("3241342142sfdd")),
        bank_code: 855, // Telebirr Code
    };

    // call the transfer method
    let result = client.transfer(transfer_payload).await;
    let result_data = match result {
        Ok(response) => {
            println!("Transfer done: {:#?}", response);
            response.data
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            None
        }
    };

    // if transfer was successful, verify it
    if let Some(tx_ref) = result_data {
        // since the transfer is done successfully, let's verify it
        let verify_result = client.verify_transfer(&tx_ref).await;
        match verify_result {
            Ok(verify_response) => {
                println!("Transfer verified: {:#?}", verify_response);
            }
            Err(e) => eprintln!("Error verifying transfer: {:#?}", e),
        }
    }
}
