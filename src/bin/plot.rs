use std::fs;
use memmap;
use clap::Parser;
use std::io::Write;
use std::path::Path;
use std::collections::{
    HashSet,
    BTreeMap
};
use serde_json;
use colored::Colorize;

use shared_object_density::args::plot::*;
use shared_object_density::types::*;

fn checkpoint_to_epoch(checkpoint: &u64, epoch2checkpoint: &BTreeMap<usize, Epoch>) -> Result<usize, String> {
    let mut found = false;
    let mut result: usize = 0;

    for (epoch, epoch_data) in epoch2checkpoint.into_iter() {
        if *checkpoint >= epoch_data.start_checkpoint as u64 && *checkpoint <= epoch_data.end_checkpoint as u64 {
            found = true;
            result = *epoch;
            break;
        }
    }

    if found == false {
        panic!("{}", format!("Could not convert checkpoint {} into epoch", *checkpoint).red());
    } else {
        Ok(result)
    }
}

fn main() {
    let args = Args::parse();

    let results_dir = Path::new("results");
    let  epoch2checkpoint_file = fs::File::open(results_dir.join("epoch2checkpoint.json")).
        expect("File not found!");

    let epoch2checkpoint_json: BTreeMap<usize, Epoch> = 
        serde_json::from_reader(epoch2checkpoint_file).
        expect("JSON was not properly formatted!");

    let data_dir = Path::new("data");
    let mut data_files: Vec<_> = fs::read_dir(data_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    data_files.sort_by_key(|f| f.metadata().unwrap().modified().unwrap());

    let mut epoch: usize;
    let mut unique_shared_objects_per_epoch: BTreeMap<usize, HashSet<String>> = BTreeMap::new();
    let mut epochs_data: BTreeMap<usize, EpochData> = BTreeMap::new();

    // auxiliary variables to calculate contention level
    let mut counts_per_interval: BTreeMap<u64, IntervalCounts> = args
        .intervals
        .iter()
        .map(|i| (*i, IntervalCounts {
            num_txs: 0,
            unique_shared_objects_per_interval: HashSet::new(),
            unique_shared_objects_touched_by_more_than_1tx_per_interval: HashSet::new(),
        }))
        .collect();
    // TODO: delete
    let mut z: HashSet<std::path::PathBuf> = HashSet::new();

    for (i, data_file) in data_files.iter().enumerate() {
        print!("\rWorking on file {}/{}...", i + 1, data_files.len());
        let _ = std::io::stdout().flush();

        let file = fs::File::open(data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let json: ResultData = serde_json::from_str(content).unwrap();

        for (checkpoint, checkpoint_data) in json.checkpoints.into_iter() {
            // convert the checkpoint number into epoch
            epoch = checkpoint_to_epoch(&checkpoint, &epoch2checkpoint_json).unwrap();
            // TODO: delete
            if epoch == 23 {
                z.insert(data_file.path());
            }

            // insert a new value for key "epoch" if it does not already exist
            unique_shared_objects_per_epoch
                .entry(epoch)
                .or_insert(HashSet::new());
            epochs_data
                .entry(epoch)
                .or_insert(EpochData {
                    num_txs_total: 0,
                    num_txs_touching_shared_objs: 0,
                    density: 0.0,
                    num_shared_objects: 0,
                    num_checkpoints: 0,
                    avg_interval_data: args.intervals.iter().map(|i| (*i, AvgIntervalData{
                        contention_degree: 0.0,
                        obj_touchability: 0.0,
                    })).collect(),
                });

            //if checkpoint > epoch2checkpoint_json.
            //        get(&epoch).unwrap().end_checkpoint.try_into().unwrap() {
            //    // The epoch ends: calculate metrics per epoch
            //    //
            //    // TODO: delete this test later
            //    if epochs_data.get(&epoch).unwrap().num_txs_total as u64 - 
            //             epoch2checkpoint_json.get(&epoch).unwrap().tx_number as u64 != 0 {
            //        println!("Epoch {:2}: {}", epoch, epochs_data.get(&epoch).unwrap().num_txs_total as u64 - 
            //             epoch2checkpoint_json.get(&epoch).unwrap().tx_number as u64);
            //    }

            //    // Calculate density as the ratio of the number of TXs touching
            //    // shared objects to the total number of TXs per epoch
            //    epochs_data.get_mut(&epoch).unwrap().density = 
            //        epochs_data.get(&epoch).unwrap().num_txs_touching_shared_objs as f64 /
            //        epochs_data.get(&epoch).unwrap().num_txs_total as f64;

            //    for interval in &args.intervals {
            //        // Calculate contention degree as the sum of contention degrees
            //        // for all intervals within that epoch divided by the number of intervals
            //        epochs_data
            //            .get_mut(&epoch)
            //            .unwrap()
            //            .avg_interval_data
            //            .get_mut(&interval)
            //            .unwrap() 
            //            .contention_degree
            //                /= epochs_data.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;

            //        // Calculate object touchebility as the sum of object touchabilities
            //        // for all intervals within that epoch divided by the number of intervals
            //        epochs_data
            //            .get_mut(&epoch)
            //            .unwrap()
            //            .avg_interval_data
            //            .get_mut(&interval)
            //            .unwrap() 
            //            .obj_touchability
            //                /= epochs_data.get(&epoch).unwrap().num_checkpoints as f64 / *interval as f64;

            //        // update contention degree counters at the end of epoch
            //        counts_per_interval
            //            .get_mut(&interval)
            //            .unwrap()
            //            .num_txs = 0;
            //        counts_per_interval
            //            .get_mut(&interval)
            //            .unwrap()
            //            .unique_shared_objects_per_interval.clear();
            //        counts_per_interval
            //            .get_mut(&interval)
            //            .unwrap()
            //            .unique_shared_objects_touched_by_more_than_1tx_per_interval.clear();
            //    }

            //    // Update the number of shared objects
            //    epochs_data.get_mut(&epoch).unwrap().num_shared_objects = unique_shared_objects_per_epoch.len();

            //    // proceed to the next epoch
            //    epoch += 1;
            //    unique_shared_objects_per_epoch.clear();
            //    epochs_data.insert(epoch, EpochData {
            //        num_txs_total: 0,
            //        num_txs_touching_shared_objs: 0,
            //        density: 0.0,
            //        num_shared_objects: 0,
            //        num_checkpoints: 0,
            //        avg_interval_data: args.intervals.iter().map(|i| (*i, AvgIntervalData{
            //            contention_degree: 0.0,
            //            obj_touchability: 0.0,
            //        })).collect(),
            //    });
            //}

            // Update the total number of TXs
            epochs_data.get_mut(&epoch).unwrap().num_txs_total += 
                checkpoint_data.num_txs_total;

            // Update the number of TXs touching shared objects
            epochs_data.get_mut(&epoch).unwrap().num_txs_touching_shared_objs += 
                checkpoint_data.num_txs_touching_shared_objs;

            // Update the number of checkpoints
            epochs_data.get_mut(&epoch).unwrap().num_checkpoints += 1;

            for (obj_id, tx_list) in checkpoint_data.shared_objects.into_iter() {
                // collect unique shared objects
                unique_shared_objects_per_epoch
                    .get_mut(&epoch)
                    .unwrap()
                    .insert(obj_id.clone());

                for interval in &args.intervals {
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .num_txs += tx_list.len() as u64;
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects_per_interval.insert(obj_id.clone());
                    if tx_list.len() > 1 {
                        counts_per_interval
                            .get_mut(&interval)
                            .unwrap()
                            .unique_shared_objects_touched_by_more_than_1tx_per_interval.insert(obj_id.clone());
                    }
                }
            }

            for interval in &args.intervals {
                // do this every `interval` checkpoints
                if (checkpoint + 1) % interval == 0 {
                    // Calculate contention degree as the number of TXs touching shared
                    // objects divided by the number of unique touched shared objects
                    let x: f64 = counts_per_interval.get(&interval).unwrap().num_txs as f64 / 
                        counts_per_interval.get(&interval).unwrap().unique_shared_objects_per_interval.len() as f64;

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
                        .unique_shared_objects_touched_by_more_than_1tx_per_interval.len() as f64 /
                        counts_per_interval.get(&interval).unwrap().unique_shared_objects_per_interval.len() as f64;

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
                    // and object touchability calculations
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .num_txs = 0;
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects_per_interval.clear();
                    counts_per_interval
                        .get_mut(&interval)
                        .unwrap()
                        .unique_shared_objects_touched_by_more_than_1tx_per_interval.clear();
                }
            }
        }
    }
    println!();
    // TODO: delete
    println!("{:?}", z);

    // Calculate metrics per epoch
    for (epoch, epoch_data) in epochs_data.iter_mut() {
        // TODO: delete this test later
        if epoch_data.num_txs_total as i64 - 
                 epoch2checkpoint_json.get(&epoch).unwrap().tx_number as i64 != 0 {
            println!("Epoch {:2}: {}", epoch, epoch_data.num_txs_total as i64 - 
                 epoch2checkpoint_json.get(&epoch).unwrap().tx_number as i64);
            println!("Computed: {}", epoch_data.num_txs_total); 
            println!("Explorer: {}", epoch2checkpoint_json.get(&epoch).unwrap().tx_number);
            println!();
        }

        // Calculate density as the ratio of the number of TXs touching
        // shared objects to the total number of TXs per epoch
        epoch_data.density = 
            epoch_data.num_txs_touching_shared_objs as f64 /
            epoch_data.num_txs_total as f64;

        for interval in &args.intervals {
            // Calculate contention degree as the sum of contention degrees
            // for all intervals within that epoch divided by the number of intervals
            epoch_data
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .contention_degree
                    /= epoch_data.num_checkpoints as f64 / *interval as f64;

            // Calculate object touchebility as the sum of object touchabilities
            // for all intervals within that epoch divided by the number of intervals
            epoch_data
                .avg_interval_data
                .get_mut(&interval)
                .unwrap() 
                .obj_touchability
                    /= epoch_data.num_checkpoints as f64 / *interval as f64;

            // update contention degree counters at the end of epoch
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .num_txs = 0;
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .unique_shared_objects_per_interval.clear();
            counts_per_interval
                .get_mut(&interval)
                .unwrap()
                .unique_shared_objects_touched_by_more_than_1tx_per_interval.clear();
        }

        // Calculate the number of unique shared objects per epoch
        epoch_data
            .num_shared_objects = unique_shared_objects_per_epoch
                .get(epoch)
                .unwrap()
                .len();
    }

    let _ = fs::write(results_dir.join("plotme.json"), serde_json::to_string_pretty(&epochs_data).
            unwrap());
    //println!("{:#?}", epochs_data);
}
