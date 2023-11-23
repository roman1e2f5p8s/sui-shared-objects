use std::fs;
use memmap;
use clap::Parser;
use std::io::Write;
use std::path::Path;
use serde_json;
use colored::Colorize;
//use std::process::exit;
use std::str::FromStr;
use std::collections::{
    BTreeSet,
    BTreeMap,
};
use indexmap::IndexMap;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::{
    TransactionDigest,
};
use sui_sdk::rpc_types::{
    SuiTransactionBlockResponseOptions,
    SuiTransactionBlockData,
    SuiTransactionBlockKind,
    SuiCommand,
};

use shared_object_density::args::density::*;
use shared_object_density::types::*;
use shared_object_density::consts::{
    DATA_DIR,
};

//#[derive(Debug)]
//struct MoveCallData {
//    count: u64,
//    senders: BTreeSet<String>,
//}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let workspace_dir = Path::new(DATA_DIR).join(args.workspace.clone());
    let mut epoch_data_files: Vec<_> = fs::read_dir(workspace_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    epoch_data_files.sort_by_key(|f| f.path());

    let state_object_id = String::from("0xf9ff3ef935ef6cdfb659a203bf2754cebeb63346e29114a535ea6f41315e5a3f");

    // count the number of TXs that touch State shared object
    let mut num_txs_touched_state = 0;

    // count the number of TXs that touch State shared object by mutable reference
    let mut num_txs_touched_state_by_mut_ref = 0;

    let mut txs_to_scan: BTreeSet<String> = BTreeSet::new();

    println!();
    for (epoch, epoch_data_file) in epoch_data_files.iter().enumerate() {
        print!("\rWorking on epoch {}...", format!("{}/{}", epoch, epoch_data_files.len() - 1).blue());
        let _ = std::io::stdout().flush();

        let file = fs::File::open(epoch_data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let result: ResultData = serde_json::from_str(content).unwrap();

        // ignore incomplete epoch data files
        if result.num_txs_scanned != result.num_txs_in_epoch {
            println!("{}", format!("\nIgnoring incomplete epoch data file {:?}\n", epoch_data_file.path()).yellow());
            break;
        }

        for (_, checkpoint_data) in result.checkpoints.into_iter() {
            let txs_touched_state = checkpoint_data
                    .shared_objects
                    .get(&state_object_id);
            if txs_touched_state != None {
                // number of TXs that touch State at this checkpoint
                let num_txs = txs_touched_state
                    .unwrap()
                    .len();

                // update the number of TXs that touch State
                num_txs_touched_state += num_txs;

                for (tx_id, mutates) in txs_touched_state.unwrap().into_iter() {
                    if *mutates {
                        num_txs_touched_state_by_mut_ref += 1;
                        txs_to_scan.insert(tx_id.to_string());
                    }
                }// end of iterating over TXs within a checkpoint
            }
        } // end of iterating over checkpoints
    } // end of iteration over epoch data files
    println!();

    println!("Total number of TXs touched State:            {:?}", num_txs_touched_state);
    println!("Total number of TXs touched State by mut ref: {:?}", num_txs_touched_state_by_mut_ref);

    let txs_to_scan_list: Vec<_> = txs_to_scan
        .iter()
        .map(|x| TransactionDigest::from_str(x).unwrap())
        .collect();

    // get TX blocks for object State and checkpoint `checkpoint_has_max_txs_touching_clock`
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("{}", format!("\n --- Sui mainnet version: {} --- \n", sui.api_version()).green());

    let mut options = SuiTransactionBlockResponseOptions::new();
    options.show_input = true;

    let total_num_txs = txs_to_scan_list.len();
    let mut num_txs_scanned = 0;
    let mut sender_set: BTreeSet<String> = BTreeSet::new();
    let mut all_move_calls: BTreeMap<String, u64> = BTreeMap::new();

    println!("{}", "TXs that touch State shared object by mut ref:".green());
    while num_txs_scanned < total_num_txs {
        let left = num_txs_scanned;
        let right;
        if left + 50 < total_num_txs {
            right = left + 50;
        } else {
            right = total_num_txs;
        }

        let tx_blocks = sui
            .read_api()
            .multi_get_transactions_with_options((&txs_to_scan_list[left..right]).to_vec(), options.clone())
            .await?;

        for tx_block in tx_blocks.iter() {
            let SuiTransactionBlockData::V1(tx_data_v1) = &tx_block.transaction.clone().unwrap().data;
            let sender = tx_data_v1.sender.to_string();
            sender_set.insert(sender.clone());
            print!("TX: {:>44}, Sender: {}", tx_block.digest.to_string(), sender);

            if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = &tx_data_v1.transaction {
                let mut move_calls: BTreeMap<String, u64> = BTreeMap::new();
                for cmd in &prog_tx.commands {
                    match cmd {
                        SuiCommand::MoveCall(c) => {
                            //print!("{}, ", format!("{}.{}", c.module, c.function).yellow());
                            move_calls
                                .entry(format!("{}.{}", c.module, c.function))
                                .or_insert(0);
                            *move_calls
                                .get_mut(&format!("{}.{}", c.module, c.function))
                                .unwrap() += 1;

                            all_move_calls
                                .entry(format!("{}::{}::{}", c.package, c.module, c.function))
                                .or_insert(0);
                            *all_move_calls
                                .get_mut(&format!("{}::{}::{}", c.package, c.module, c.function))
                                .unwrap() += 1;
                            //all_move_calls
                            //    .get_mut(&format!("{}::{}::{}", c.package, c.module, c.function))
                            //    .unwrap()
                            //    .senders
                            //    .insert(sender.clone());
                        }
                        SuiCommand::MakeMoveVec(_, _) => {
                            //print!("MakeMoveVec, ");
                        }
                        SuiCommand::TransferObjects(_, _) => {
                            //print!("{:#?}, {:#?}, ", objs, addr);
                        }
                        SuiCommand::SplitCoins(_, _) => {
                            //print!("{:#?}, ", coin);
                        }
                        SuiCommand::MergeCoins(_, _) => {
                            //print!("{:#?}, {:#?}, ", target, coins);
                        }
                        SuiCommand::Publish(_) => {
                            //print!("{:#?}, ", deps);
                        }
                        SuiCommand::Upgrade(_, _, _) => {
                            //print!("{:#?}, {:#?}, {:#?}, ", deps, current_package_id, ticket);
                        }
                    }
                }
                print!(", Calls: ");
                for (k, v) in move_calls.iter() {
                    print!("{} x {}, ", k, v);
                }
                println!();
                move_calls.clear();
            }
            //exit(0);
        }

        num_txs_scanned += tx_blocks.len();
    }
    assert_eq!(num_txs_scanned, total_num_txs);
    println!();

    let mut all_move_calls_vec = Vec::from_iter(all_move_calls);
    all_move_calls_vec.sort_by(|(_, a), (_, b)| b.cmp(&a));
    let sorted_all_move_calls: IndexMap<String, u64> = all_move_calls_vec
        .into_iter()
        .collect();
    println!("Move calls that modify State object:");
    println!("{:#?}", sorted_all_move_calls);

    println!("{}", "Done!".green());
    Ok(())
}
