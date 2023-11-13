use std::fs;
use memmap;
use clap::Parser;
use std::io::Write;
use std::path::Path;
use serde_json;
use colored::Colorize;
//use std::process::exit;

use shared_object_density::args::density::*;
use shared_object_density::types::*;
use shared_object_density::consts::{
    DATA_DIR,
    RESULTS_DIR,
};


fn main() {
    let args = Args::parse();

    let workspace_dir = Path::new(DATA_DIR).join(args.workspace.clone());
    let mut epoch_data_files: Vec<_> = fs::read_dir(workspace_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    epoch_data_files.sort_by_key(|f| f.path());

    let mut num_txs_touched_clock = 0;
    let mut num_txs_touched_clock_and_other_shared_obj = 0;
    let mut num_txs_touched_only_clock_shared_obj = 0;
    let clock_object_id = String::from("0x0000000000000000000000000000000000000000000000000000000000000006");

    println!();
    for (epoch, epoch_data_file) in epoch_data_files.iter().enumerate() {
        print!("\rWorking on file {}...", format!("{}/{}", epoch + 1, epoch_data_files.len()).blue());
        let _ = std::io::stdout().flush();
        println!();

        let file = fs::File::open(epoch_data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let result: ResultData = serde_json::from_str(content).unwrap();

        // ignore incomplete epoch data files
        if result.num_txs_scanned != result.num_txs_in_epoch {
            println!("{}", format!("\nIgnoring incomplete epoch data file {:?}\n", epoch_data_file.path()).yellow());
            continue;
        }

        for (_, checkpoint_data) in result.checkpoints.into_iter() {
            let txs_touched_clock = checkpoint_data
                    .shared_objects
                    .get(&clock_object_id);
            if txs_touched_clock != None {
                num_txs_touched_clock += txs_touched_clock
                    .unwrap()
                    .len();
                for (tx_id, _) in txs_touched_clock.unwrap().into_iter() {
                    let mut n = 0;
                    for (obj_id, tx_list) in checkpoint_data.shared_objects.clone().into_iter() {
                        if obj_id != clock_object_id {
                            if tx_list.get(&tx_id.clone()) != None {
                                num_txs_touched_clock_and_other_shared_obj += 1;
                                break; // found the same TX touching Clock and other shared obj
                            } else {
                                n += 1;
                            }
                        }
                    }
                    if n == checkpoint_data.shared_objects.len() - 1 {
                        num_txs_touched_only_clock_shared_obj += 1;
                        println!("TX {:?} touches only Clock shared object", tx_id);
                    }
                }// end of iterating over objects within a checkpoint
            }
        } // end of iterating over checkpoints

    } // end of iteration over epoch data files
    println!();
    println!("Total number of TXs touched Clock:                         {:?}", num_txs_touched_clock);
    println!("Total number of TXs touched Clock and other shared object: {:?}", num_txs_touched_clock_and_other_shared_obj);
    println!("Total number of TXs touched only Clock shared object:      {:?}", num_txs_touched_only_clock_shared_obj);
    assert_eq!(num_txs_touched_clock, num_txs_touched_clock_and_other_shared_obj + num_txs_touched_only_clock_shared_obj);

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
    //let _ = fs::write(results_dir.join(PLOT_FILENAME), serde_json::to_string_pretty(&epochs_data).
    //        unwrap());

    println!("{}", "Done!".green());
}
