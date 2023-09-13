use std::fs;
use std::path::Path;
use serde_json;
use clap::{Parser, ValueEnum};
use std::io::Write;
use serde::Serialize;
use std::str::FromStr;
// use std::process::exit;
use std::collections::HashSet;
use std::collections::HashMap;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::rpc_types::SuiTransactionBlockData;
use sui_sdk::rpc_types::SuiTransactionBlockKind;
use sui_sdk::rpc_types::SuiCallArg;
use sui_sdk::rpc_types::SuiObjectArg;
use sui_sdk::rpc_types::SuiTransactionBlock;
// use sui_sdk::rpc_types::SuiTransactionBlockResponse;
use sui_sdk::rpc_types::SuiTransactionBlockResponseQuery;
// use sui_sdk::rpc_types::SuiObjectDataOptions;

// from which TX to start to query;
// the corresponding TX won't be included!
// const CURSOR: &str = "CP5xMb2EdVzbBjAeoTQypSg5ADeRHJ9qtpyszKBnH56H";
// 9oG3Haf35Ew6wbWumt7xbPG3vcqnpQTaMMadQWNJEWcY";

/// Estimate how often Sui transactions operate with shared objects
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
struct Args {
    /// Which network to use
    #[arg(short, long, value_enum, default_value_t = NetworkType::Mainnet)]
    network: NetworkType,

    /// Number of TXs to scan, >= 0
    #[arg(short, long, default_value_t = 1000)]
    tx_number: usize,

    /// Digest of TX from which to start scanning.
    /// Note that the corresponding TX won't be scaned!
    #[arg(short, long)]
    cursor: String,
}

#[derive(ValueEnum, Debug, Clone)]
enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
}

#[derive(Debug)]
struct SharedObjInfo {
    id: String,
    mutable: bool
}

#[derive(Debug)]
struct TxInfo {
    num_total: usize,
    num_shared: usize,
    shared_objects: Vec<SharedObjInfo>
}

#[derive(Debug, Serialize)]
struct TxMutInfo {
    tx_id: String,
    mutable: bool
}


// print type of variable
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// Given Option<sui_json_rpc_types::sui_transaction::SuiTransactionBlock>
// for TX, return its inputs
fn process_tx_inputs(tx_block: &Option<SuiTransactionBlock>) -> TxInfo {
    // `tx_block` should have structure like this:
    // Some(SuiTransactionBlock {
    //  data: V1(SuiTransactionBlockDataV1 { 
    //      transaction: ProgrammableTransaction(SuiProgrammableTransactionBlock { 
    //      inputs: [
    // So, we need to unwrap Some, and get the `data` field of SuiTransactionBlock.
    // Then, we access the V1 variant of the SuiTransactionBlockData enum.
    // There is only one variant, so we don't need `if let`
    let SuiTransactionBlockData::V1(tx_data_v1) = &tx_block.as_ref().unwrap().data;

    // Now, get the `transaction` field of the SuiTransactionBlockDataV1 struct,
    // then access the ProgrammableTransaction variant of 
    // the SuiTransactionBlockKind enum
    if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = &tx_data_v1.transaction {
        // to count the number of shared mutable objects
        let mut count = 0;
        let mut shared_objects: Vec<SharedObjInfo> = Vec::new();

        for input in prog_tx.inputs.iter() {
            // input has type of sui_sdk::rpc_types::SuiCallArg;
            // the sui_sdk::rpc_types::SuiCallArg enum has two variants:
            // Object and Pure. We need only Objects.
            if let SuiCallArg::Object(obj) = input {
                // obj has type of sui_sdk::rpc_types::SuiObjectArg;
                // sui_sdk::rpc_types::SuiObjectArg enum has two variants:
                // ImmOrOwnedObject and SharedObject. We need only SharedObject
                if let SuiObjectArg::SharedObject{object_id, mutable, ..} = obj {
                    count = count + 1;
                    shared_objects.push(SharedObjInfo {
                        id: object_id.to_string(),
                        mutable: *mutable
                    })
                }
            }
        }
        // println!("Total: {}, Owned: {}, Shared: {}",
        //          prog_tx.inputs.len(), prog_tx.inputs.len() - count, count);
        return TxInfo {
            num_total: prog_tx.inputs.len(),
            num_shared: count,
            shared_objects: shared_objects
        };
    }
    TxInfo {
        num_total: 0,
        num_shared: 0,
        shared_objects: Vec::new()
    }
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build(format!("https://fullnode.{:?}.sui.io:443", args.network))
        .await
        .unwrap();
    println!("\nSui {:?} version: {}\n", args.network, sui.api_version());

    // TX options indicate what info to be included in the response
    let mut txs_options = SuiTransactionBlockResponseOptions::new();
    txs_options.show_input = true;

    let query = SuiTransactionBlockResponseQuery::new(None, Some(txs_options));

    // from which TX to start the query.
    // The response will not include this TX.
    // Set to None to get the latest TXs
    let mut cursor = TransactionDigest::from_str(&args.cursor)?;

    // count the numebr of TX analyzed
    let mut tx_count = 0;

    // count the number of TX touching 0 shared objects
    let mut tx_0shared_count = 0;

    // count the number of TX touching 0 objects
    let mut tx_0total_count = 0;

    // Map for storing data we are interested in.
    // It has the following structure:
    // {
    //      checkpoint:
    //      {
    //          SharedObjID:
    //          [
    //              (TX_ID, mutable),
    //              ...
    //          ]
    //          ...
    //      ...
    //      }
    // }
    let mut data: HashMap<u64, HashMap<String, Vec<TxMutInfo>>> = HashMap::new();

    while tx_count < args.tx_number {
        // The result will have type of sui_json_rpc_types::Page<
        // sui_json_rpc_types::sui_transaction::SuiTransactionBlockResponse,
        // sui_types::digests::TransactionDigest>
        let txs_blocks = sui
            .read_api()
            .query_transaction_blocks(query.clone(), Some(cursor), Some(args.tx_number), true)
            .await?;

        // println!("Number of TXs: {}", txs_blocks.data.len());
        // println!("Has next page: {}", txs_blocks.has_next_page);
        // println!("Next cursor: {}", txs_blocks.next_cursor.unwrap().to_string());
        // println!();

        for tx in txs_blocks.data.iter() {
            // println!("TX: {}", tx.digest.to_string());
            let tx_info = process_tx_inputs(&tx.transaction);
            if tx_info.num_shared == 0 {
                tx_0shared_count = tx_0shared_count + 1;
            } else {
                for shared_obj in tx_info.shared_objects.iter() {
                    // insert a new checkpoint if it does not exist already
                    data.
                        entry(tx.checkpoint.unwrap_or_default()).
                        or_insert(HashMap::new());

                    // insert a new shared object ID if it does not exist already
                    let _ = *data.
                        get_mut(&tx.checkpoint.unwrap_or_default()).
                        unwrap().
                        entry(shared_obj.id.clone()).
                        or_insert(Vec::new());

                    // both checkpoint and shared object ID keys must now exist,
                    // so we can update the list of TX operating with that shared object
                    let _ = data.
                        get_mut(&tx.checkpoint.unwrap_or_default()).
                        unwrap().
                        get_mut(&shared_obj.id).
                        unwrap().
                        push(TxMutInfo {
                            tx_id: tx.digest.to_string(),
                            mutable: shared_obj.mutable
                        });
                }
            }
            if tx_info.num_total == 0 {
                tx_0total_count = tx_0total_count + 1;
            }
            // println!("Timestamp: {:?}", tx.timestamp_ms.unwrap_or_default());
            // println!();
        }

        // exit(0);

        tx_count = tx_count + txs_blocks.data.len();
        cursor = txs_blocks.next_cursor.unwrap();
        print!("\rNumber of TX analyzed : {}/{} ...", tx_count, args.tx_number);
        let _ = std::io::stdout().flush();
        // break;
    }
    println!();

    // store the start and the end cursor: to reproduce the results
    // and to continue scanning if necessary
    println!("Start cursor: {}", args.cursor);
    println!("End   cursor: {}", cursor.to_string());

    // save data to disk
    let dir = Path::new("data");
    fs::create_dir_all(dir)?;
    fs::write(dir.join("data.json"), serde_json::to_string_pretty(&data).unwrap())?;

    println!();
    // println!("{:#?}", data);
    for (checkpoint, obj_map) in data.into_iter() {
        println!("Checkpoint: {}", checkpoint);
        let mut txs = HashSet::new();
        for (obj_id, tx_list) in obj_map.into_iter() {
            println!("Obj {} touched by {} TXs", obj_id, tx_list.len());
            for tx in tx_list.iter() {
                txs.insert(tx.tx_id.clone());
            }
        }
        println!("Shared-object TX count: {}", txs.len());
        println!();
    }

    println!("Total number of TX analyzed : {}", tx_count);
    println!("Total number of TX requested: {}", args.tx_number);
    println!("Total number of TX touching 0 shared objects: {}", tx_0shared_count);
    println!("Total number of TX touching 0 objects: {}", tx_0total_count);

    Ok(())
}
