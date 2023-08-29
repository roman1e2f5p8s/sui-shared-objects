use std::str::FromStr;
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::rpc_types::SuiTransactionBlockData;
use sui_sdk::rpc_types::SuiTransactionBlockKind;
use sui_sdk::rpc_types::SuiCallArg;
use sui_sdk::rpc_types::SuiObjectArg;
use sui_sdk::rpc_types::SuiTransactionBlockResponseQuery;

// One of mainnet, testnet, devnet
const NETWORK: &str = "mainnet";
// TX digest as string (to test for one TX)
const TX_DIGEST: &str = "E2hjN5qEWHFuW1wYSL2KqFpWKbfgYPGoxUHzxYgbkzLV";
// whether execution status is important for us or not
const TX_STATUS_NEEDED: bool = true;
// how many TXs to query
const LIMIT: usize = 10;
// from which TX to start to query;
// the corresponding TX won't be included!
const CURSOR: &str = "CutJJwAGNNBDwDyjHyc4VgCRbzoujGUFh72muh94pP1J";

// print type of variable
// fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build(format!("https://fullnode.{}.sui.io:443", NETWORK),)
        .await
        .unwrap();
    println!("\nSui {} version: {}\n", NETWORK, sui.api_version());


    let mut txs_options = SuiTransactionBlockResponseOptions::new();
    // txs_options.show_input = true;
    if TX_STATUS_NEEDED == true {
        // effects are needed to get exec status
        txs_options.show_effects = true;
    }
    let query = SuiTransactionBlockResponseQuery::new(None, Some(txs_options));
    let cursor = TransactionDigest::from_str(CURSOR)?;

    // The result will have type of sui_json_rpc_types::Page<
    // sui_json_rpc_types::sui_transaction::SuiTransactionBlockResponse,
    // sui_types::digests::TransactionDigest>
    let txs_blocks = sui
        .read_api()
        .query_transaction_blocks(query, Some(cursor), Some(LIMIT), true)
        .await?;
    // println!("{:?}", txs_blocks);
    println!("Number of TXs: {}", txs_blocks.data.len());
    println!("Has next page: {}", txs_blocks.has_next_page);
    println!("Next cursor: {:?}", txs_blocks.next_cursor.unwrap());
    println!();

    for tx in txs_blocks.data.iter() {
        // skip TXs whose execution status is false
        if TX_STATUS_NEEDED == true && tx.status_ok().clone().unwrap() == false {
            continue;
        }
        println!("Digest: {:?}", tx.digest);
        // println!("TX: {:?}", tx.transaction);
        println!("Checkpoint: {:?}", tx.checkpoint.unwrap_or_default());
        println!("Timestamp: {:?}", tx.timestamp_ms.unwrap_or_default());
        // Sui execution status
        if TX_STATUS_NEEDED == true {
            println!("Status OK: {:?}", tx.status_ok().unwrap());
        }
        println!();
        // break;
    }


    // Convert TX digest from string to the corresponding struct
    let tx_digest = TransactionDigest::from_str(TX_DIGEST)?;

    // We don't need all the TX info, just its inputs
    let tx_options = SuiTransactionBlockResponseOptions::new()
        .with_input();

    // Return TX with specified options based on its digest
    // The returned value would look like this:
    // SuiTransactionBlockResponse {
    //      digest: TransactionDigest(TX_DIGEST),
    //      transaction: Some(SuiTransactionBlock {
    //          data: V1(SuiTransactionBlockDataV1 {
    //              transaction: ProgrammableTransaction(SuiProgrammableTransactionBlock {
    //                  inputs: []...
    // So we will need to unwrap it later
    let tx = sui
        .read_api()
        .get_transaction_with_options(tx_digest, tx_options)
        .await?;
    // println!("TX data:\n{:?}\n", tx);

    // We need to get into the 'transaction' field of SuiTransactionBlockResponse,
    // then unwrap Some, and get the 'data' field of SuiTransactionBlock.
    // Then, we access the V1 variant of the SuiTransactionBlockData enum.
    // There is only one variant, so we don't need `if let`
    let SuiTransactionBlockData::V1(tx_data_v1) = tx.transaction.unwrap().data;

    // Get the `transaction` field of the SuiTransactionBlockDataV1 struct,
    // then access the ProgrammableTransaction variant of 
    // the SuiTransactionBlockKind enum
    if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = tx_data_v1.transaction {
        // Finally, get the list of TX inputs
        let inputs = prog_tx.inputs;
        // println!("Number of inputs: {}\n", inputs.len());

        // count the number of shared mutable objects
        let mut count = 0;

        for input in inputs.iter() {
            // input has type of sui_sdk::rpc_types::SuiCallArg;
            // the sui_sdk::rpc_types::SuiCallArg enum has two variants:
            // Object and Pure. We need only Objects.
            // TODO: check with someone diffs between ImmObj and immutable SharedObj
            // TODO: can one TX have an immutable shared obj and another one the same obj as
            // mutable?
            if let SuiCallArg::Object(obj) = input {
                // obj has type of sui_sdk::rpc_types::SuiObjectArg;
                // sui_sdk::rpc_types::SuiObjectArg enum has two variants:
                // ImmOrOwnedObject and SharedObject. We need only mutable SharedObject
                if let SuiObjectArg::SharedObject{object_id, mutable, ..} = obj {
                    if *mutable == true {
                        count = count + 1;
                        // println!("Shared mutable object ID: {}", object_id);
                    }
                }
            }
        }
        // println!("\nNumber of shared objects touched by TX: {}\n", count);
    }

    Ok(())
}
