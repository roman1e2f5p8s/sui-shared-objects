use std::fs;
use std::path::Path;
use std::str::FromStr;
use clap::Parser;
use serde_json;
use memmap;
use std::io::Write;
use colored::Colorize;
use std::collections::{
    HashSet,
    BTreeMap,
};
use tokio::time::{
    sleep,
    Duration,
};
//use std::process::exit;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::{
    TransactionFilter,
    SuiTransactionBlockResponseQuery,
    SuiTransactionBlockResponseOptions,
};

use sui_shared_objects::args::query_txs::Args;
use sui_shared_objects::utils::process_tx_inputs;
use sui_shared_objects::consts::{
    RESULTS_DIR,
    EPOCH_TO_CHECKPOINTS_FILENAME,
};
use sui_shared_objects::types::{
    EpochToCheckpointData,
    CheckpointData,
    ResultData,
};


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

    // Indicates which filters and options to apply while querying
    let query = SuiTransactionBlockResponseQuery::new(None, Some(txs_options.clone()));

    // from which TX to start the query.
    // The response will not include this TX.
    let mut cursor = None;

    // count the numebr of TX analyzed
    let mut tx_count = 0;

    // count the number of TX touching 0 shared objects
    let mut tx_0shared_count = 0;

    // count the number of TX touching 0 objects
    let mut tx_0total_count = 0;

    // If this number exceeds args.retry_number, terminate the program and save data.
    // Otherwise, sleep some time and retry query.
    let mut retry_number = 0;
    
    // repeat query is transaction or checkpoint field is None
    let mut repeat_query_on_none = false;

    // get information about the epoch of interest
    let results_dir = Path::new(RESULTS_DIR);
    let epoch2checkpoint_file = fs::File::open(results_dir.join(EPOCH_TO_CHECKPOINTS_FILENAME))
        .expect("File not found!");
    let epoch2checkpoint_data: EpochToCheckpointData = 
        serde_json::from_reader(epoch2checkpoint_file)
        .expect("JSON was not properly formatted!");
    let epoch_data = epoch2checkpoint_data
        .epochs
        .get(&args.epoch)
        .unwrap();

    // use transaction filter to query the 1st TX at the start checkpoint of the epoch
    let checkpoint_query = SuiTransactionBlockResponseQuery::new(
        Some(TransactionFilter::Checkpoint(epoch_data.start_checkpoint as u64)), Some(txs_options));

    // number of TXs left to scan
    let mut tx_to_scan = epoch_data.tx_number;

    // create workspace if it does not exist yet
    let workspace_dir = Path::new("data").join(args.workspace);
    if workspace_dir.exists() {
        if args.verbose {
            println!("{}", format!("Workspace \"{}\" already exists\n", workspace_dir.display()).green());
        }
    } else {
        fs::create_dir_all(workspace_dir.clone())?;
        if args.verbose {
            println!("{}", format!("Created new workspace \"{}\"\n", workspace_dir.display()).blue());
        }
    }

    // Map (sorted by key) for storing data we are interested in.
    // result.checkpints has the following structure:
    // {
    //      checkpoint:
    //      {
    //          num_txs_total: ...,
    //          num_txs_touching_shared_objs: ...,
    //          shared_objects: {
    //              SharedObjID:
    //              {
    //                  TX_ID: mutates,
    //                  ...
    //              }
    //              ...
    //          }
    //      ...
    //      }
    // }
    let mut result = ResultData {
        network: args.network.to_string(),
        version: sui.api_version().to_string(),
        epoch: args.epoch,
        num_txs_in_epoch: epoch_data.tx_number,
        start_checkpoint: epoch_data.start_checkpoint,
        end_checkpoint: epoch_data.end_checkpoint,
        last_cursor: String::new(),
        num_txs_scanned: 0,
        num_txs_touching_0_shared_objs: 0,
        num_txs_touching_0_objs: 0,
        checkpoints: BTreeMap::new(),
    };

    // epoch data file to save data
    let epoch_data_file = workspace_dir.join(format!("epoch={:0>3}_{}-{}.json",
        args.epoch,
        epoch_data.start_checkpoint,
        epoch_data.end_checkpoint
    ));
    // check if the epoch data file already exists
    if epoch_data_file.exists() {
        println!("{}", format!("File \"{}\" already exists.", epoch_data_file.display()).yellow());

        // read it
        let mmap = unsafe {memmap::Mmap::map(&fs::File::open(epoch_data_file.clone()).unwrap())}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        result = serde_json::from_str(content).unwrap();

        // check if  this epoch data file is complete, i.e., if all TXs for 
        // the epoch of interest were scanned
        if result.num_txs_scanned == result.num_txs_in_epoch {
            println!("{}", format!("All TXs for epoch {} were scanned. Exiting with Ok\n", args.epoch).green());
            return Ok(());
        } else {
            println!("{}", format!("Continue scanning the rest of TXs for epoch {}\n", args.epoch).blue());
            cursor = Some(TransactionDigest::from_str(&result.last_cursor)?);
            tx_count = result.num_txs_scanned;
            tx_to_scan = epoch_data.tx_number - tx_count;
            tx_0shared_count = result.num_txs_touching_0_shared_objs;
            tx_0total_count = result.num_txs_touching_0_objs;
        }
    } else {
        'outer: while {
            if repeat_query_on_none == true {
                repeat_query_on_none = false;
            }

            let tx_block = match sui.read_api().query_transaction_blocks(
                    checkpoint_query.clone(), None, Some(1), false).await {
                Ok(block) => {
                    retry_number = 0;
                    block
                },
                Err(error) => {
                    println!("\n  {}: {:?}", "ERROR".red(), error);
                    if retry_number < args.retry_number {
                        for i in 0..args.retry_sleep {
                            print!("{}", format!("\r    Retrying query #{}/{} for the 1st checkpoint ({}) of epoch {} in {} s..", retry_number + 1,
                                args.retry_number, epoch_data.start_checkpoint, args.epoch, args.retry_sleep - i).yellow());
                            std::io::stdout().flush()?;
                            sleep(Duration::from_secs(1)).await;
                        }
                        print!("{}", format!("\r    Retrying query #{}/{} for the 1st checkpoint ({}) of epoch {} in {} s..", retry_number + 1,
                            args.retry_number, epoch_data.start_checkpoint, args.epoch, 0).yellow());
                        retry_number += 1;
                        println!();
                        continue 'outer;
                    } else {
                        println!("{}", format!("    Retry number is reached, terminating the program").yellow());
                        break 'outer;
                    }
                },
            };

            assert_eq!(tx_block.data.len(), 1);

            // Check if there is no block with transaction: None.
            // If exists, repeat query for the same checkpoint
            for tx in tx_block.data.iter() {
                if tx.transaction.as_ref().is_none() {
                    if args.verbose {
                        println!("\n{}\n", "None TX encountered. Repeating query again".yellow());
                    }
                    repeat_query_on_none = true;
                    break;
                }
                if tx.checkpoint.is_none() {
                    if args.verbose {
                        println!("\n{}\n", "None checkpoint encountered. Repeating query again".yellow());
                    }
                    repeat_query_on_none = true;
                    break;
                }
            }
            if repeat_query_on_none == true {
                sleep(Duration::from_secs(1)).await;
                continue 'outer;
            }

            for tx in tx_block.data.iter() {
                // insert a new checkpoint if it does not exist already
                result.checkpoints.
                    entry(tx.checkpoint.unwrap_or_default()).
                    or_insert(CheckpointData {
                        num_txs_total: 0,
                        num_txs_touching_shared_objs: 0,
                        shared_objects: BTreeMap::new()
                    });
                result.checkpoints.
                    get_mut(&tx.checkpoint.unwrap_or_default()).
                    unwrap().
                    num_txs_total += 1;

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
                            or_insert(BTreeMap::new());

                        // both checkpoint and shared object ID keys must now exist,
                        // so we can update the list of TX operating with that shared object
                        let _ = result.checkpoints
                            .get_mut(&tx.checkpoint.unwrap_or_default())
                            .unwrap()
                            .shared_objects
                            .get_mut(&shared_obj.id)
                            .unwrap()
                            .entry(tx.digest.to_string())
                            .or_insert(shared_obj.mutable);
                    }
                }
                if tx_info.num_total == 0 {
                    tx_0total_count = tx_0total_count + 1;
                }
            }

            tx_to_scan = tx_to_scan - 1;
            tx_count = tx_count + 1;
            cursor = tx_block.next_cursor;

            print!("\rNumber of TX analyzed : {}...", format!("{}/{}", tx_count, epoch_data.tx_number).blue());
            std::io::stdout().flush()?;

            // condition to break the loop
            false
        } { }
    }

    // continue querying for the rest of the epoch
    retry_number = 0;
    repeat_query_on_none = false;

    'outer: while {
        if repeat_query_on_none == true {
            repeat_query_on_none = false;
        }

        // If Ok, the result will have type of sui_json_rpc_types::Page<
        // sui_json_rpc_types::sui_transaction::SuiTransactionBlockResponse,
        // sui_types::digests::TransactionDigest>
        let txs_blocks = match sui.read_api().query_transaction_blocks(
                query.clone(), cursor, Some(tx_to_scan), false).await {
            Ok(blocks) => {
                retry_number = 0;
                blocks
            },
            Err(error) => {
                println!("\n  {}: {:?}", "ERROR".red(), error);
                if retry_number < args.retry_number {
                    for i in 0..args.retry_sleep {
                        print!("{}", format!("\r    Retrying query #{}/{} starting at cursor {} in {} s..", retry_number + 1,
                            args.retry_number, cursor.unwrap().to_string(), args.retry_sleep - i).yellow());
                        std::io::stdout().flush()?;
                        sleep(Duration::from_secs(1)).await;
                    }
                    print!("{}", format!("\r    Retrying query #{}/{} starting at cursor {} in {} s   ", retry_number + 1,
                        args.retry_number, cursor.unwrap().to_string(), 0).yellow());
                    retry_number += 1;
                    println!();
                    continue 'outer;
                } else {
                    println!("{}", format!("    Retry number is reached, saving data and terminating the program").yellow());
                    break 'outer;
                }
            },
        };

        // Check if there is no block with transaction: None.
        // If exists, repeat query for the same cursor
        for tx in txs_blocks.data.iter() {
            if tx.transaction.as_ref().is_none() {
                if args.verbose {
                    println!("\n{}\n", "None TX encountered. Repeating query again".yellow());
                }
                repeat_query_on_none = true;
                break;
            }
            if tx.checkpoint.is_none() {
                if args.verbose {
                    println!("\n{}\n", "None checkpoint encountered. Repeating query again".yellow());
                }
                repeat_query_on_none = true;
                break;
            }
        }
        if repeat_query_on_none == true {
            sleep(Duration::from_secs(1)).await;
            continue 'outer;
        }

        // println!("Next cursor: {}", txs_blocks.next_cursor.unwrap().to_string());
        // println!("{:?}", txs_blocks);
        // exit(0);
        for tx in txs_blocks.data.iter() {
            // insert a new checkpoint if it does not exist already
            result.checkpoints.
                entry(tx.checkpoint.unwrap_or_default()).
                or_insert(CheckpointData {
                    num_txs_total: 0,
                    num_txs_touching_shared_objs: 0,
                    shared_objects: BTreeMap::new()
                });
            result.checkpoints.
                get_mut(&tx.checkpoint.unwrap_or_default()).
                unwrap().
                num_txs_total += 1;

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
                        or_insert(BTreeMap::new());

                    // both checkpoint and shared object ID keys must now exist,
                    // so we can update the list of TX operating with that shared object
                    let _ = result.checkpoints
                        .get_mut(&tx.checkpoint.unwrap_or_default())
                        .unwrap()
                        .shared_objects
                        .get_mut(&shared_obj.id)
                        .unwrap()
                        .entry(tx.digest.to_string())
                        .or_insert(shared_obj.mutable);
                }
            }
            if tx_info.num_total == 0 {
                tx_0total_count = tx_0total_count + 1;
            }
        }

        tx_to_scan = tx_to_scan - txs_blocks.data.len();
        tx_count = tx_count + txs_blocks.data.len();
        cursor = txs_blocks.next_cursor;

        print!("\rNumber of TX analyzed : {}...", format!("{}/{}", tx_count, epoch_data.tx_number).blue());
        std::io::stdout().flush()?;

        // condition to break the loop
        tx_count < epoch_data.tx_number
    } { }
    println!();

    if tx_count == epoch_data.tx_number {
        println!("{}", format!("All TXs for epoch {} were scanned.", args.epoch).green());
    } else {
        println!("{}", format!("Not all TXs for epoch {} were scanned!", args.epoch).yellow());
        println!("{}", format!("Please repeat the query for epoch {} again.", args.epoch).yellow());
    }

    result.last_cursor = cursor.unwrap().to_string();
    result.num_txs_scanned = tx_count;
    result.num_txs_touching_0_shared_objs = tx_0shared_count;
    result.num_txs_touching_0_objs = tx_0total_count;

    // save data to disk
    fs::write(epoch_data_file, serde_json::to_string_pretty(&result).unwrap())?;

    println!();
    if args.verbose {
        for (checkpoint, obj_map) in result.checkpoints.into_iter() {
            println!("Checkpoint: {}", checkpoint);
            let mut txs = HashSet::new();
            for (obj_id, tx_list) in obj_map.shared_objects.into_iter() {
                println!("Obj {} touched by {} TXs", obj_id, tx_list.len());
                for (tx_id, _) in tx_list.into_iter() {
                    txs.insert(tx_id);
                }
            }
            println!("Shared-object TX count: {}", txs.len());
            println!();
        }
    }

    println!("{}", format!("Total number of TXs in epoch {:>3}: {}", args.epoch, epoch_data.tx_number).green());
    println!("{}", format!("Total number of TXs scanned     : {}", tx_count).green());
    println!("{}", format!("Total number of TXs touching 0 shared objects: {}", tx_0shared_count).green());
    println!("{}", format!("Total number of TXs touching 0        objects: {}", tx_0total_count).green());

    Ok(())
}
