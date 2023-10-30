use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;

const TX_DIGEST_STR: &str = "7K65LrBLwTeMdzWd4WYd3Kibybp21Ss7QP2f8MBhHti2";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let tx_digest = TransactionDigest::from_str(TX_DIGEST_STR)?;
    let options = SuiTransactionBlockResponseOptions::new().with_input();

    let tx_block = sui
        .read_api()
        .get_transaction_with_options(tx_digest, options)
        .await?;
    println!("{:#?}", tx_block);

    Ok(())
}
