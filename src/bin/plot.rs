use std::fs;
use memmap;
use clap::Parser;
use std::io::Write;
use std::path::Path;
use std::collections::{
    HashSet,
    BTreeSet,
    BTreeMap,
};
use serde_json;
//use colored::Colorize;
//use std::process::exit;

use shared_object_density::args::plot::*;
use shared_object_density::types::*;

fn main() {
    let args = Args::parse();

    let workspace_dir = Path::new("data").join(args.workspace);
    let mut epoch_data_files: Vec<_> = fs::read_dir(workspace_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    epoch_data_files.sort_by_key(|f| f.path());

    let mut unique_shared_objects_per_epoch: BTreeMap<usize, HashSet<String>> = BTreeMap::new();
    let mut unique_shared_objects_total: BTreeSet<String> = BTreeSet::new();
    let mut epochs_data: BTreeMap<usize, EpochData> = BTreeMap::new();

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

    for (epoch, epoch_data_file) in epoch_data_files.iter().enumerate() {
        print!("\rWorking on file {}/{}...", epoch + 1, epoch_data_files.len());
        let _ = std::io::stdout().flush();

        let file = fs::File::open(epoch_data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let result: ResultData = serde_json::from_str(content).unwrap();

        // insert a new value for key "epoch"
        unique_shared_objects_per_epoch
            .entry(epoch)
            .or_insert(HashSet::new());
        epochs_data
            .entry(epoch)
            .or_insert(EpochData {
                num_txs_total: 0,
                num_txs_touching_shared_objs: 0,
                density: 0.0,
                num_shared_objects_per_epoch: 0,
                num_shared_objects_total: 0,
                num_checkpoints: 0,
                avg_interval_data: args.intervals.iter().map(|i| (*i, AvgIntervalData{
                    contention_degree: 0.0,
                    obj_touchability: 0.0,
                })).collect(),
            });

        // Update the number of checkpoints
        epochs_data.get_mut(&epoch).unwrap().num_checkpoints = result.end_checkpoint - result.start_checkpoint + 1;

        for (checkpoint, checkpoint_data) in result.checkpoints.into_iter() {
            // Update the total number of TXs
            epochs_data.get_mut(&epoch).unwrap().num_txs_total += 
                checkpoint_data.num_txs_total;

            // Update the number of TXs touching shared objects
            epochs_data.get_mut(&epoch).unwrap().num_txs_touching_shared_objs += 
                checkpoint_data.num_txs_touching_shared_objs;

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
                
                // collect unique shared in the Sui network
                unique_shared_objects_total
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
            } // end of iterating over objects within a checkpoint

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
        assert_eq!(epochs_data.get(&epoch).unwrap().num_txs_total, result.num_txs_scanned); 

        // Calculate metrics per epoch
        //
        // Calculate density as the ratio of the number of TXs touching
        // shared objects to the total number of TXs per epoch
        epochs_data.get_mut(&epoch).unwrap().density = 
            epochs_data.get(&epoch).unwrap().num_txs_touching_shared_objs as f64 /
            epochs_data.get(&epoch).unwrap().num_txs_total as f64;
        //
        for interval in &args.intervals {
            // Calculate contention degree as the sum of contention degrees
            // for all intervals within that epoch divided by the number of intervals
            epochs_data
                .get_mut(&epoch)
                .unwrap()
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .contention_degree
                    /= epochs_data.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;
            //
            // Calculate object touchebility as the sum of object touchabilities
            // for all intervals within that epoch divided by the number of intervals
            epochs_data
                .get_mut(&epoch)
                .unwrap()
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .obj_touchability
                    /= epochs_data.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;
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
            .get_mut(&epoch)
            .unwrap()
            .num_shared_objects_per_epoch = unique_shared_objects_per_epoch
                .get(&epoch)
                .unwrap()
                .len();
        //
        // Calculate the number of unique shared in the Sui network
        epochs_data
            .get_mut(&epoch)
            .unwrap()
            .num_shared_objects_total = unique_shared_objects_total
                .len();
    } // end of iteration over epoch data files
    println!();

    let results_dir = Path::new("results");
    let _ = fs::write(results_dir.join("plotme.json"), serde_json::to_string_pretty(&epochs_data).
            unwrap());
    let _ = fs::write(results_dir.join("shared_objects_set.json"), serde_json::to_string_pretty(&unique_shared_objects_total).
            unwrap());
}
