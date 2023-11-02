use std::fs;
use memmap;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::collections::{
    BTreeSet,
};
use serde_json;
use clap::Parser;
use tokio::time::{
    sleep,
    Duration,
};
use colored::Colorize;
//use std::process::exit;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::rpc_types::SuiObjectDataOptions;

use shared_object_density::args::query_obj::Args;
use shared_object_density::consts::{
    RESULTS_DIR,
    QUERY_MAX_RESULT_LIMIT,
    SHARED_OBJECTS_SET_FILENAME,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // read the shared objects set data file
    let results_dir = Path::new(RESULTS_DIR);
    let file = fs::File::open(results_dir.join(SHARED_OBJECTS_SET_FILENAME).as_path())
        .expect("File not found!");
    let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
    let content = std::str::from_utf8(&mmap).unwrap();
    let shared_objects: BTreeSet<String> = serde_json::from_str(content).unwrap();

    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build(format!("https://fullnode.{:?}.sui.io:443", args.network))
        .await
        .unwrap();
    println!("{}", format!("\n --- Sui {:?} version: {} --- \n", args.network, sui.api_version()).green());

    // options indicate which info about shared objects should
    // be included in the response
    let mut options = SuiObjectDataOptions::new();
    options.show_content = true;

    // store the total number of objects
    let num_objects = shared_objects.len();

    // convert the set of shared object to a vector of shared object IDs
    let shared_objects_ids: Vec<ObjectID> = shared_objects
        .into_iter()
        .map(|s| ObjectID::from_str(&s).unwrap())
        .collect();

    // count the number of scanned objects
    let mut scanned_objects_count = 0;

    // bounds to slice the vector of shared object IDs
    let mut left = 0;
    let mut right = QUERY_MAX_RESULT_LIMIT;
    if num_objects < QUERY_MAX_RESULT_LIMIT {
        right = num_objects;
    }

    // If this number exceeds args.retry_number, terminate the program and save data.
    // Otherwise, sleep some time and retry query.
    let mut retry_number = 0;
    
    // repeat query is transaction or checkpoint field is None
    let mut repeat_query_on_none = false;

    'outer: while {
        if repeat_query_on_none == true {
            repeat_query_on_none = false;
        }

        // TODO
        // If Ok, the result should have entries of structure like this:
        // SuiObjectResponse {
        //     data: Some(
        //         SuiObjectData {
        //             object_id: ...,
        //             version: SequenceNumber(...),
        //             digest: ...,
        //             type_: None,
        //             owner: None,
        //             previous_transaction: None,
        //             storage_rebate: None,
        //             display: None,
        //             content: Some(
        //                 MoveObject(
        //                     SuiParsedMoveObject {
        //                         type_: StructTag {
        //                             address: package address,
        //                             module: Identifier("sui_system"),    <- THIS
        //                             name: Identifier("SuiSystemState"),  <- THIS
        //                             type_params: [],
        //                         },
        //                         has_public_transfer: false,              <- THIS
        //                         ),
        //                     },
        //                 ),
        //             ),
        //             bcs: None,
        //         },
        //     ),
        //     error: None,
        // }
        // We need to get the field marked with THIS
        let objects = match sui.read_api().multi_get_object_with_options(
                (&shared_objects_ids[left..right]).to_vec(), options.clone()).await {
            Ok(objects) => {
                retry_number = 0;
                objects
            },
            Err(error) => {
                println!("\n  {}: {:?}", "ERROR".red(), error);
                if retry_number < args.retry_number {
                    for i in 0..args.retry_sleep {
                        print!("{}", format!("\r    Retrying query #{} starting at index {} in {} s..", retry_number + 1,
                            scanned_objects_count, args.retry_sleep - i).yellow());
                        std::io::stdout().flush()?;
                        sleep(Duration::from_secs(1)).await;
                    }
                    print!("{}", format!("\r    Retrying query #{} starting at index {} in {} s   ", retry_number + 1,
                        scanned_objects_count, 0).yellow());
                    retry_number += 1;
                    println!();
                    continue 'outer;
                } else {
                    println!("{}", format!("    Retry number is reached, saving data and terminating the program").yellow());
                    break 'outer;
                }
            },
        };
        //println!("{:#?}", objects);

        // TODO: process objects here

        // update count of scanned shared objects
        scanned_objects_count += objects.len();

        // update bounds
        left = right;
        right += QUERY_MAX_RESULT_LIMIT;
        if right > num_objects {
            right = num_objects;
        }
        
        print!("\rNumber of objects analyzed : {}...", format!("{}/{}", scanned_objects_count, num_objects).blue());
        std::io::stdout().flush()?;

        // condition to break the loop
        scanned_objects_count < num_objects
    } { }
    println!();

    // number of scanned objects must be equal to the number of objects in datafile
    assert_eq!(scanned_objects_count, num_objects);

    println!("{}", "Done!".green());
    Ok(())
}
