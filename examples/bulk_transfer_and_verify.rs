use chapa_rust::{
    client::ChapaClient,
    config::ChapaConfigBuilder,
    models::transfer::{BulkData, BulkTransferOptions},
};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();
    // initialize a chapa client
    let config = ChapaConfigBuilder::new().build().unwrap();
    let mut client = ChapaClient::from_config(config).unwrap();

    // Note: Make sure you give real value to BulkData fields
    let bulk_transfer_option = BulkTransferOptions {
        title: String::from("This Month Salary!"),
        currency: String::from("ETB"),
        bulk_data: vec![
            BulkData {
                account_name: Some(String::from("Israel Goytom")),
                account_number: String::from("09xxxxxxxx"),
                amount: String::from("1"),
                reference: Some(String::from("b1111124")),
                bank_code: 128,
            },
            BulkData {
                account_name: Some(String::from("Israel Goytom")),
                account_number: String::from("09xxxxxxxx"),
                amount: String::from("1"),
                reference: Some(String::from("b2222e5r")),
                bank_code: 128,
            },
        ],
    };

    // call the get_transactions method
    let result = client.bulk_transfer(bulk_transfer_option).await;
    let bulk_response = match result {
        Ok(bulk) => {
            println!("{:#?}", bulk);
            bulk.data
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            None
        }
    };

    if let Some(data) = bulk_response {
        let verify_result = client.verify_bulk_transfer(&data.id.to_string()).await;
        match verify_result {
            Ok(verify_response) => {
                println!("Bulk Transfer verified: {:#?}", verify_response);
            }
            Err(e) => eprintln!("Error verifying bulk transfer: {:#?}", e),
        }
    }
}
