mod args;

use std::fs;
use std::path::Path;
use clap::Parser;
use serde_json;
use std::io::Write;
use serde::Serialize;
use std::str::FromStr;
// use std::process::exit;
use std::collections::HashSet;
use std::collections::{HashMap, BTreeMap};

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
use crate::args::Args;

// from which TX to start to query;
// the corresponding TX won't be included!
// const CURSOR: &str = "CP5xMb2EdVzbBjAeoTQypSg5ADeRHJ9qtpyszKBnH56H";
// 9oG3Haf35Ew6wbWumt7xbPG3vcqnpQTaMMadQWNJEWcY";

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
    mutates: bool
}

#[derive(Debug, Serialize)]
struct CheckpointData {
    num_txs_total: usize,
    num_txs_touching_shared_objs: usize,
    shared_objects: HashMap<String, Vec<TxMutInfo>>
}

#[derive(Debug, Serialize)]
struct ResultData {
    start_cursor: String,
    end_cursor: String,
    descending: bool,
    num_txs_scanned: usize,
    num_txs_touching_0_shared_objs: usize,
    num_txs_touching_0_objs: usize,
    checkpoints: BTreeMap<u64, CheckpointData>
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
    let mut cursor = None;
    if !args.cursor.is_empty() {
        cursor = Some(TransactionDigest::from_str(&args.cursor)?);
    }

    // count the numebr of TX analyzed
    let mut tx_count = 0;

    // number of TXs left to scan
    let mut tx_to_scan = args.tx_number;

    // count the number of TX touching 0 shared objects
    let mut tx_0shared_count = 0;

    // count the number of TX touching 0 objects
    let mut tx_0total_count = 0;

    // Map (sorted by key) for storing data we are interested in.
    // result.checkpints has the following structure:
    // {
    //      checkpoint:
    //      {
    //          num_txs_total: ...,
    //          num_txs_touching_shared_objs: ...,
    //          shared_objects: {
    //              SharedObjID:
    //              [
    //                  (TX_ID, mutates),
    //                  ...
    //              ]
    //              ...
    //          }
    //      ...
    //      }
    // }
    let mut result = ResultData {
        start_cursor: args.cursor.clone(),
        end_cursor: String::from(""),
        descending: args.descending,
        num_txs_scanned: 0,
        num_txs_touching_0_shared_objs: 0,
        num_txs_touching_0_objs: 0,
        checkpoints: BTreeMap::new(),
    };

    // let checkpoints = sui
    //     .read_api()
    //     .get_checkpoints(Some(10.into()), Some(2), false)
    //     .await?;
    // println!("{:?}", checkpoints);

    while {
        // The result will have type of sui_json_rpc_types::Page<
        // sui_json_rpc_types::sui_transaction::SuiTransactionBlockResponse,
        // sui_types::digests::TransactionDigest>
        let txs_blocks = sui
            .read_api()
            .query_transaction_blocks(query.clone(), cursor, 
                                      Some(tx_to_scan), args.descending)
            .await?;

        // println!("Number of TXs: {}", txs_blocks.data.len());
        // println!("Has next page: {}", txs_blocks.has_next_page);
        // println!("Next cursor: {}", txs_blocks.next_cursor.unwrap().to_string());
        // println!();
        // println!("{:?}", txs_blocks);

        for tx in txs_blocks.data.iter() {
            // println!("TX: {}", tx.digest.to_string());
            let tx_info = process_tx_inputs(&tx.transaction);

            // insert a new checkpoint if it does not exist already
            result.checkpoints.
                entry(tx.checkpoint.unwrap_or_default()).
                or_insert(CheckpointData {
                    num_txs_total: 0,
                    num_txs_touching_shared_objs: 0,
                    shared_objects: HashMap::new()
                });
            result.checkpoints.
                get_mut(&tx.checkpoint.unwrap_or_default()).
                unwrap().
                num_txs_total += 1;

            if tx_info.num_shared == 0 {
                tx_0shared_count = tx_0shared_count + 1;
            } else {
                result.checkpoints.
                    get_mut(&tx.checkpoint.unwrap_or_default()).
                    unwrap().
                    num_txs_touching_shared_objs += 1;
                for shared_obj in tx_info.shared_objects.iter() {
                    // insert a new shared object ID if it does not exist already
                    let _ = *result.checkpoints.
                        get_mut(&tx.checkpoint.unwrap_or_default()).
                        unwrap().
                        shared_objects.
                        entry(shared_obj.id.clone()).
                        or_insert(Vec::new());

                    // both checkpoint and shared object ID keys must now exist,
                    // so we can update the list of TX operating with that shared object
                    let _ = result.checkpoints.
                        get_mut(&tx.checkpoint.unwrap_or_default()).
                        unwrap().
                        shared_objects.
                        get_mut(&shared_obj.id).
                        unwrap().
                        push(TxMutInfo {
                            tx_id: tx.digest.to_string(),
                            mutates: shared_obj.mutable
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

        tx_to_scan = tx_to_scan - txs_blocks.data.len();
        tx_count = tx_count + txs_blocks.data.len();
        cursor = txs_blocks.next_cursor;
        print!("\rNumber of TX analyzed : {}/{} ...", tx_count, args.tx_number);
        std::io::stdout().flush()?;
        // break;

        // condition to break the loop
        tx_count < args.tx_number && txs_blocks.has_next_page == true
    } { }
    println!();

    // store the start and the end cursor: to reproduce the results
    // and to continue scanning if necessary
    println!("Start cursor: {}", args.cursor);
    println!("End   cursor: {}", cursor.unwrap().to_string());
    result.end_cursor = cursor.unwrap().to_string();
    result.num_txs_scanned = tx_count;
    result.num_txs_touching_0_shared_objs = tx_0shared_count;
    result.num_txs_touching_0_objs = tx_0total_count;

    // save data to disk
    let dir = Path::new("data");
    fs::create_dir_all(dir)?;
    fs::write(dir.join(format!("{}.json", if !args.cursor.is_empty() {
                args.cursor
            } else {
                if !args.descending {
                    String::from("ascending")
                } else {
                    String::from("descending")
                }
            })),
            serde_json::to_string_pretty(&result).
            unwrap())?;

    println!();
    // println!("{:#?}", result);
    if args.verbose == true {
        for (checkpoint, obj_map) in result.checkpoints.into_iter() {
            println!("Checkpoint: {}", checkpoint);
            let mut txs = HashSet::new();
            for (obj_id, tx_list) in obj_map.shared_objects.into_iter() {
                println!("Obj {} touched by {} TXs", obj_id, tx_list.len());
                for tx in tx_list.iter() {
                    txs.insert(tx.tx_id.clone());
                }
            }
            println!("Shared-object TX count: {}", txs.len());
            println!();
        }
    }

    println!("Total number of TXs scanned  : {}", tx_count);
    println!("Total number of TXs requested: {}", args.tx_number);
    println!("Total number of TXs touching 0 shared objects: {}", tx_0shared_count);
    println!("Total number of TXs touching 0        objects: {}", tx_0total_count);

    Ok(())
}
