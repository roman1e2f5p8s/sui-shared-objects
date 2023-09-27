use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use serde_json;

use shared_object_density::args::plot::*;
use shared_object_density::types::*;

const EVERY_N_CHECKPOINTS: u64 = 60;

fn main() {
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

    let mut epochs_data: BTreeMap<usize, EpochData> = BTreeMap::new();
    let mut epoch: usize = 0;
    epochs_data.insert(epoch, EpochData {
        num_txs_total: 0,
        num_txs_touching_shared_objs: 0,
        density: 0.0,
        num_checkpoints: 0,
        contention_level: 0.0,
    });

    // variables to calculate contention level
    let mut num_txs = 0;
    let mut num_obj = 0;

    for data_file in data_files {
        let  file = fs::File::open(data_file.path()).
            expect("File not found!");

        println!("Reading {:?}...", data_file.path());
        let json: ResultData = 
            serde_json::from_reader(file).
            expect("JSON was not properly formatted!");
        
        println!("Working on {:?}...\n", data_file.path());
        for (checkpoint, checkpoint_data) in json.checkpoints.into_iter() {
            if checkpoint > epoch2checkpoint_json.
                    get(&epoch).unwrap().end_checkpoint.try_into().unwrap() {
                // the epoch ends: calculate metrics per epoch
                epochs_data.get_mut(&epoch).unwrap().density = 
                    epochs_data.get(&epoch).unwrap().num_txs_touching_shared_objs as f64 /
                    epochs_data.get(&epoch).unwrap().num_txs_total as f64;
                epochs_data.get_mut(&epoch).unwrap().contention_level /= 
                    epochs_data.get(&epoch).unwrap().num_checkpoints as f64 / 
                     EVERY_N_CHECKPOINTS as f64;
                // update contention level counters at the end of epoch
                num_txs = 0;
                num_obj = 0;

                // proceed to the next epoch
                epoch += 1;
                epochs_data.insert(epoch, EpochData {
                    num_txs_total: 0,
                    num_txs_touching_shared_objs: 0,
                    density: 0.0,
                    num_checkpoints: 0,
                    contention_level: 0.0,
                });
            }
            epochs_data.get_mut(&epoch).unwrap().num_txs_total += 
                checkpoint_data.num_txs_total;
            epochs_data.get_mut(&epoch).unwrap().num_txs_touching_shared_objs += 
                checkpoint_data.num_txs_touching_shared_objs;
            epochs_data.get_mut(&epoch).unwrap().num_checkpoints += 1;

            for (_, tx_list) in checkpoint_data.shared_objects.into_iter() {
                num_txs += tx_list.len();
                num_obj += 1;
            }

            // do this every EVERY_N_CHECKPOINTS checkpoints
            if (checkpoint + 1) % EVERY_N_CHECKPOINTS == 0 {
                let x: f64 = num_txs as f64 / num_obj as f64;
                if !x.is_nan() {
                    epochs_data.get_mut(&epoch).unwrap().contention_level += x;
                }
                // renew counters
                num_txs = 0;
                num_obj = 0;
            }
        }
    }
    let _ = fs::write(results_dir.join("plot.json"), serde_json::to_string_pretty(&epochs_data).
            unwrap());
    println!("{:#?}", epochs_data);
}
