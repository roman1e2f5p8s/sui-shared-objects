use std::fs;
use memmap;
use clap::Parser;
use std::io::Write;
use std::path::Path;
use std::collections::{
    HashSet,
    BTreeMap,
};
use serde_json;
use colored::Colorize;
// use std::process::exit;

use sui_shared_objects::args::metrics::*;
use sui_shared_objects::types::*;
use sui_shared_objects::consts::{
    DATA_DIR,
    RESULTS_DIR,
    PLOT_FILENAME,
    SHARED_OBJECTS_SET_FILENAME,
};

fn main() {
    let args = Args::parse();

    let workspace_dir = Path::new(DATA_DIR).join(args.workspace.clone());
    let mut epoch_data_files: Vec<_> = fs::read_dir(workspace_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    epoch_data_files.sort_by_key(|f| f.path());

    let mut unique_shared_objects_per_epoch: BTreeMap<usize, HashSet<String>> = BTreeMap::new();
    let mut unique_shared_objects_total = SharedObjectsSetData {
        shared_objects: BTreeMap::new(),
    };
    let mut epochs_data = EpochsData {
        epochs: BTreeMap::new(),
    };

    let mut txs_touching_at_least_one_shared_obj_by_mut: HashSet<String> = HashSet::new();
    let mut tx_shared_obj_count_per_checkpoint: BTreeMap<String, u64> = BTreeMap::new();

    // auxiliary variables to calculate contention level
    let mut counts_per_interval: BTreeMap<u64, IntervalCounts> = args
        .intervals
        .iter()
        .map(|i| (*i, IntervalCounts {
            num_txs: 0,
            unique_shared_objects: HashSet::new(),
            unique_shared_objects_touched_by_more_than_1tx: HashSet::new(),
        }))
        .collect();

    println!("{}", "\n --- Calculating epoch data --- \n".green());
    for (k, epoch_data_file) in epoch_data_files.iter().enumerate() {
        let epoch: usize = epoch_data_file.path().to_str().unwrap().split("=").collect::<Vec<_>>()[1][..3].parse().unwrap();

        print!("\rWorking on epoch {}...", format!("{}: {}/{}", epoch, k, epoch_data_files.len() - 1).blue());
        let _ = std::io::stdout().flush();

        let file = fs::File::open(epoch_data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let result: ResultData = serde_json::from_str(content).unwrap();

        // ignore incomplete epoch data files
        if result.num_txs_scanned != result.num_txs_in_epoch {
            println!("{}", format!("\nIgnoring incomplete epoch data files starting from {:?}\n", epoch_data_file.path()).yellow());
            break;
        }

        // insert a new value for key "epoch"
        unique_shared_objects_per_epoch
            .entry(epoch)
            .or_insert(HashSet::new());
        epochs_data
            .epochs
            .entry(epoch)
            .or_insert(EpochData {
                num_txs_total: 0,
                num_txs_touching_shared_objs: 0,
                num_txs_touching_at_least_one_shared_obj_by_mut: 0,
                num_shared_objects_per_tx: 0.0,
                density: 0.0,
                density_mut: 0.0,
                num_shared_objects_per_epoch: 0,
                num_shared_objects_total: 0,
                num_checkpoints: 0,
                avg_interval_data: args.intervals.iter().map(|i| (*i, AvgIntervalData{
                    contention_degree: 0.0,
                    obj_touchability: 0.0,
                })).collect(),
            });

        // Update the number of checkpoints
        epochs_data
            .epochs
            .get_mut(&epoch)
            .unwrap()
            .num_checkpoints = result.end_checkpoint - result.start_checkpoint + 1;

        for (checkpoint, checkpoint_data) in result.checkpoints.into_iter() {
            // Update the total number of TXs
            epochs_data
                .epochs
                .get_mut(&epoch)
                .unwrap()
                .num_txs_total += checkpoint_data.num_txs_total;

            // Update the number of TXs touching shared objects
            epochs_data
                .epochs
                .get_mut(&epoch)
                .unwrap()
                .num_txs_touching_shared_objs += checkpoint_data.num_txs_touching_shared_objs;

            // Count the number of unique TXs touching shared objects per interval.
            // It might look that this number will be the same for all intervals,
            // however it is not because some will be re-initialized later
            for interval in &args.intervals {
                counts_per_interval
                    .get_mut(&interval)
                    .unwrap()
                    .num_txs += checkpoint_data.num_txs_touching_shared_objs as u64; // TODO: tx_list.len() as u64;
            }

            for (obj_id, tx_list) in checkpoint_data.shared_objects.into_iter() {
                // collect unique shared objects per epoch
                unique_shared_objects_per_epoch
                    .get_mut(&epoch)
                    .unwrap()
                    .insert(obj_id.clone());
                
                // for every interval, update the sets of unique shared objects
                for interval in &args.intervals {
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects.insert(obj_id.clone());
                    if tx_list.len() > 1 {
                        counts_per_interval
                            .get_mut(&interval)
                            .unwrap()
                            .unique_shared_objects_touched_by_more_than_1tx.insert(obj_id.clone());
                    }
                }

                // collect unique shared in the Sui network and some data about them
                //
                // insert the key if it does not exist yet
                unique_shared_objects_total
                    .shared_objects
                    .entry(obj_id.clone())
                    .or_insert(SharedObjectSetData {
                        num_txs: 0,
                        num_mut_refs: 0,
                        first_touched_at_epoch: epoch,
                        first_touched_at_checkpoint: checkpoint,
                        first_touched_by_txs: tx_list.clone(),
                    });
                //
                // update the total number of TXs that touched that object
                unique_shared_objects_total
                    .shared_objects
                    .get_mut(&obj_id)
                    .unwrap()
                    .num_txs += tx_list.len();
                //
                // update the total number of how many times that object
                // was passed by a mutable reference
                for (tx_id, mut_ref) in tx_list.into_iter() {
                    if mut_ref {
                        unique_shared_objects_total
                            .shared_objects
                            .get_mut(&obj_id)
                            .unwrap()
                            .num_mut_refs += 1;

                        // collect unique txs that touch at least one shared object by mut ref
                        txs_touching_at_least_one_shared_obj_by_mut.insert(tx_id.clone());
                    }
                    // insert new entry with TX ID and set count to 0
                    tx_shared_obj_count_per_checkpoint
                        .entry(tx_id.clone())
                        .or_insert(0);
                    *tx_shared_obj_count_per_checkpoint
                        .get_mut(&tx_id.clone())
                        .unwrap() += 1;
                } // end of iterating over TX list
            } // end of iterating over objects within a checkpoint

            // Update the average (accumulative) number of per TX
            if tx_shared_obj_count_per_checkpoint.len() > 0 {
                epochs_data
                    .epochs
                    .get_mut(&epoch)
                    .unwrap()
                    .num_shared_objects_per_tx += tx_shared_obj_count_per_checkpoint.values().sum::<u64>() as f64 / tx_shared_obj_count_per_checkpoint.len() as f64;
            }
            // clear the map for the next checkpoint
            tx_shared_obj_count_per_checkpoint.clear();

            // Update the number of TXs touching at least one shared object by mut ref
            epochs_data
                .epochs
                .get_mut(&epoch)
                .unwrap()
                .num_txs_touching_at_least_one_shared_obj_by_mut += txs_touching_at_least_one_shared_obj_by_mut.len();
            // clear set for the next checkpoint
            txs_touching_at_least_one_shared_obj_by_mut.clear();

            for interval in &args.intervals {
                // do this every `interval` checkpoints
                if (checkpoint - result.start_checkpoint as u64 + 1) % interval == 0 {
                    // Calculate contention degree as the number of TXs touching shared
                    // objects divided by the number of unique touched shared objects
                    let x: f64 = counts_per_interval.get(&interval).unwrap().num_txs as f64 / 
                        counts_per_interval.get(&interval).unwrap().unique_shared_objects.len() as f64;

                    if !x.is_nan() {
                        // Sum up interval contention degree to the epoch contention degree,
                        // which will be averaged when epoch ends
                        epochs_data
                            .epochs
                            .get_mut(&epoch)
                            .unwrap()
                            .avg_interval_data
                            .get_mut(&interval)
                            .unwrap() 
                            .contention_degree
                                += x;
                    }

                    // Calculate object touchability as the number of objects touched by
                    // more than one TX divided by the number of shared objects
                    let y: f64 = counts_per_interval.get(&interval).unwrap()
                        .unique_shared_objects_touched_by_more_than_1tx.len() as f64 /
                        counts_per_interval.get(&interval).unwrap().unique_shared_objects.len() as f64;

                    if !y.is_nan() {
                        // Sum up interval object touchability to the epoch object touchability,
                        // which will be averaged when epoch ends
                        epochs_data
                            .epochs
                            .get_mut(&epoch)
                            .unwrap()
                            .avg_interval_data
                            .get_mut(&interval)
                            .unwrap() 
                            .obj_touchability
                                += y;
                    }

                    // renew counters and clear sets needed for contention degree
                    // and object touchability calculations for that interval
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .num_txs = 0;
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects.clear();
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects_touched_by_more_than_1tx.clear();
                } // end of if(checkpoint - result.start_checkpoint + 1 % interval == 0)
            } // end of iterating over intervals
        } // end of iterating over checkpoints

        // total number of scanned TXs per epoch must be equal to the sum of TXs from
        // all checkpoints for that epoch
        assert_eq!(epochs_data.epochs.get(&epoch).unwrap().num_txs_total, result.num_txs_scanned); 
        assert!(epochs_data.epochs.get(&epoch).unwrap().num_txs_touching_at_least_one_shared_obj_by_mut <=
            epochs_data.epochs.get(&epoch).unwrap().num_txs_touching_shared_objs);

        // Calculate metrics per epoch
        //
        // average number of shared objects per transaction
        epochs_data
            .epochs
            .get_mut(&epoch)
            .unwrap()
            .num_shared_objects_per_tx /= epochs_data.epochs.get(&epoch).unwrap().num_checkpoints as f64;
        // Calculate density as the ratio of the number of TXs touching
        // shared objects to the total number of TXs per epoch
        epochs_data.epochs.get_mut(&epoch).unwrap().density = 
            epochs_data.epochs.get(&epoch).unwrap().num_txs_touching_shared_objs as f64 /
            epochs_data.epochs.get(&epoch).unwrap().num_txs_total as f64;
        //
        // Calculate mut-density as the ratio of the number of TXs touching
        // at least one shared object by mut ref to the total number of TXs per epoch
        epochs_data.epochs.get_mut(&epoch).unwrap().density_mut = 
            epochs_data.epochs.get(&epoch).unwrap().num_txs_touching_at_least_one_shared_obj_by_mut as f64 /
            epochs_data.epochs.get(&epoch).unwrap().num_txs_total as f64;
        //
        for interval in &args.intervals {
            // Calculate contention degree as the sum of contention degrees
            // for all intervals within that epoch divided by the number of intervals
            epochs_data
                .epochs
                .get_mut(&epoch)
                .unwrap()
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .contention_degree
                    /= epochs_data.epochs.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;
            //
            // Calculate object touchebility as the sum of object touchabilities
            // for all intervals within that epoch divided by the number of intervals
            epochs_data
                .epochs
                .get_mut(&epoch)
                .unwrap()
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .obj_touchability
                    /= epochs_data.epochs.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;
            //
            // update contention degree counters for the next epoch
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .num_txs = 0;
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .unique_shared_objects.clear();
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .unique_shared_objects_touched_by_more_than_1tx.clear();
        }
        //
        // Calculate the number of unique shared objects per epoch
        epochs_data
            .epochs
            .get_mut(&epoch)
            .unwrap()
            .num_shared_objects_per_epoch = unique_shared_objects_per_epoch
                .get(&epoch)
                .unwrap()
                .len();
        //
        // Calculate the number of unique shared in the Sui network
        epochs_data
            .epochs
            .get_mut(&epoch)
            .unwrap()
            .num_shared_objects_total = unique_shared_objects_total
                .shared_objects
                .len();
    } // end of iteration over epoch data files
    println!();

    // save results 
    let results_dir = Path::new(RESULTS_DIR).join(args.workspace);
    if results_dir.exists() {
        if args.verbose {
            println!("{}", format!("Workspace \"{}\" already exists\n", results_dir.display()).green());
        }
    } else {
        fs::create_dir_all(results_dir.clone()).unwrap();
        if args.verbose {
            println!("{}", format!("Created new workspace \"{}\"\n", results_dir.display()).blue());
        }
    }
    let _ = fs::write(results_dir.join(PLOT_FILENAME), serde_json::to_string_pretty(&epochs_data).
            unwrap());
    let _ = fs::write(results_dir.join(SHARED_OBJECTS_SET_FILENAME), serde_json::to_string_pretty(&unique_shared_objects_total).
            unwrap());

    println!("{}", "Done!".green());
}
