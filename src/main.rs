use std::fs;
use std::path::Path;
use clap::Parser;
use serde_json;
use std::io::Write;
use std::str::FromStr;
use colored::Colorize;
use std::process::exit;
use std::collections::HashSet;
use std::collections::{HashMap, BTreeMap};
use tokio::time::{sleep, Duration};

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseQuery;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;

use shared_object_density::args::query::Args;
use shared_object_density::utils::process_tx_inputs;
use shared_object_density::types::{TxMutInfo, CheckpointData, ResultData};


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build(format!("https://fullnode.{:?}.sui.io:443", args.network))
        .await
        .unwrap();
    println!("{}", format!("\n --- Sui {:?} version: {} --- \n", args.network, sui.api_version()).green());

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

    // If this number exceeds args.retry_number, terminate the program and save data.
    // Otherwise, sleep some time and retry query.
    let mut retry_number = 0;

    let mut b = false;

    'outer: while {
        if b == true {
            println!("Last cursor: {:?}", cursor);
            b = false;
        }

        // If Ok, the result will have type of sui_json_rpc_types::Page<
        // sui_json_rpc_types::sui_transaction::SuiTransactionBlockResponse,
        // sui_types::digests::TransactionDigest>
        let txs_blocks = match sui.read_api().query_transaction_blocks(
                query.clone(), cursor, Some(tx_to_scan), args.descending).await {
            Ok(blocks) => {
                retry_number = 0;
                blocks
            },
            Err(error) => {
                println!("\n  {}: {:?}", "ERROR".red(), error);
                if retry_number < args.retry_number {
                    for i in 0..args.retry_sleep {
                        print!("{}", format!("\r    Retrying query #{} starting at cursor {} in {} s..", retry_number + 1,
                            cursor.unwrap().to_string(), args.retry_sleep - i).yellow());
                        std::io::stdout().flush()?;
                        sleep(Duration::from_secs(1)).await;
                    }
                    print!("{}", format!("\r    Retrying query #{} starting at cursor {} in {} s   ", retry_number + 1,
                        cursor.unwrap().to_string(), 0).yellow());
                    retry_number += 1;
                    println!();
                    continue 'outer;
                } else {
                    println!("{}", format!("\t    Retry number is reached, saving data and terminating the program").yellow());
                    break 'outer;
                }
            },
        };

        // Check if there is no block with transaction: None.
        // If exists, repeat query for the same cursor
        for tx in txs_blocks.data.iter() {
            println!("{:?}", tx.checkpoint);
            if tx.transaction.as_ref() == None {
                println!("\n{}: {:?}", "Empty TX block".red(), tx);
                println!("{} {:?}\n", "Repeating query again for cursor:".red(), cursor);
                b = true;
                break;
            }
        }
        if b == true {
            continue 'outer;
        }

        // println!("Next cursor: {}", txs_blocks.next_cursor.unwrap().to_string());
        // println!("{:?}", txs_blocks);
        exit(0);
        for tx in txs_blocks.data.iter() {
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

            // println!("TX: {}", tx.digest.to_string());
            // if tx.transaction.as_ref() == None {
            //     println!("{}: {:#?}\n", "Empty TX block".red(), tx);
            //     println!("{:?}\n", cursor);
            //     println!("{:?}\n", txs_blocks);
            //     tx_0total_count = tx_0total_count + 1;
            //     exit(0);
            //     continue;
            // }
            let tx_info = process_tx_inputs(&tx.transaction);

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
        }

        tx_to_scan = tx_to_scan - txs_blocks.data.len();
        tx_count = tx_count + txs_blocks.data.len();
        cursor = txs_blocks.next_cursor;

        print!("\rNumber of TX analyzed : {}...", format!("{}/{}", tx_count, args.tx_number).blue());
        std::io::stdout().flush()?;

        // condition to break the loop
        tx_count < args.tx_number && txs_blocks.has_next_page == true
    } { }
    println!();

    // store the start and the end cursor: to reproduce the results
    // and to continue scanning if necessary
    println!("{}", format!("Start cursor: {}", args.cursor).green());
    println!("{}", format!("End   cursor: {}", cursor.unwrap().to_string()).green());
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

    println!("{}", format!("Total number of TXs scanned  : {}", tx_count).green());
    println!("{}", format!("Total number of TXs requested: {}", args.tx_number).green());
    println!("{}", format!("Total number of TXs touching 0 shared objects: {}", tx_0shared_count).green());
    println!("{}", format!("Total number of TXs touching 0        objects: {}", tx_0total_count).green());

    Ok(())
}
