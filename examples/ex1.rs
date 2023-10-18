use colored::Colorize;
use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::rpc_types::SuiTransactionBlockResponseQuery;

const TX_DIGEST_STR: &str = "3EfuLX6t79X63bNaDP1LeGYdgbSZB6jJq25eUibXKXqv";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("{}", format!("\n --- Sui mainnet version: {} --- \n", sui.api_version()).green());

    let tx_digest = TransactionDigest::from_str(TX_DIGEST_STR)?;
    let mut options = SuiTransactionBlockResponseOptions::new();
    options.show_input = true;
    // options.show_events = true;
    // options.show_effects = true;
    // options.show_object_changes = true;
    // options.show_raw_input = true;

    let query = SuiTransactionBlockResponseQuery::new(None, Some(options.clone()));

    let tx_block = sui
        .read_api()
        .query_transaction_blocks(query, Some(tx_digest), None, false)
        // .get_transaction_with_options(tx_digest, options.clone())
        .await?;
    // println!("{:?}", options);
    // println!("{:#?}", tx_block);

    for tx in tx_block.data.iter() {
        println!("Checkpoint: {:?}", tx.checkpoint);
        // println!("TX: {:?}", tx);
    }

    Ok(())
}
