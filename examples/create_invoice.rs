use std::env;

use btcpay::models::*;
use btcpay::*;

#[tokio::main]
async fn main() {
    let key = SecretKey::from_slice(
        &Vec::<u8>::from_hex(
            &env::var("BTCPAY_KEY").expect("BTCPAY_KEY environment variable not set"),
        )
        .unwrap(),
    )
    .unwrap();
    let keypair: KeyPair = key.into();

    let client = BTCPayClient::new(
        &env::var("BTCPAY_URL").expect("BTCPAY_URL environment variable not set"),
        keypair,
        Some(&env::var("BTCPAY_MERCHANT").expect("BTCPAY_MERCHANT environment variable not set")),
    )
    .unwrap();

    println!(
        "{:?}",
        client
            .create_invoice(CreateInvoiceArgs::new("USD", 20.0))
            .await
    );
}
