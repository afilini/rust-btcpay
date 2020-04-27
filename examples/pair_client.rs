use std::env;

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
        None,
    )
    .unwrap();

    println!(
        "{:?}",
        client
            .pair_client(
                &env::var("BTCPAY_PAIRCODE").expect("BTCPAY_PAIRCODE environment variable not set")
            )
            .await
    );
}
