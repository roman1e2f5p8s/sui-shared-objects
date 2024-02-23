use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::{
    SuiTransactionBlockResponseOptions,
    SuiTransactionBlockResponseQuery,
};

// const TX_DIGEST_STR: &str = "E98T8NJzLgEyWPqvQ6PGoHEH1PajqWdyWJkkHHWZZrNm"; // to go down
const TX_DIGEST_STR: &str = "5a6Q3kZoj4RipcwpNcbspz9z2Cyh4rmdhgvhYPgQGes6"; // to go up

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let cursor = Some(TransactionDigest::from_str(TX_DIGEST_STR)?);
    let options = SuiTransactionBlockResponseOptions::new().with_input();
    let query = SuiTransactionBlockResponseQuery::new(None, Some(options.clone()));
    let descending = false;

    let tx_blocks = sui
        .read_api()
        .query_transaction_blocks(
            query.clone(), cursor, Some(19), descending)
        .await?;
    println!("{:#?}", tx_blocks);

    Ok(())
}
