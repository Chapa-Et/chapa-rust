use chapa_rust::utils::{self, GenTxRefOptions};

#[tokio::main]
async fn main() {
    // This example demonstrates how to generate a transaction reference (tx_ref)
    let option = GenTxRefOptions::new(Some(true), Some("ORDER-".to_string()), Some(10));
    // OR simply use the default options
    // let option = GenTxRefOptions::default();
    let tx_ref = utils::generate_tx_ref(option);
    println!("Generated tx_ref: {}", tx_ref);
}
