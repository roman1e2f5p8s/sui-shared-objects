use std::fs;
use memmap;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use indexmap::IndexMap;
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
use sui_sdk::rpc_types::SuiParsedData;

use sui_shared_objects::args::query_obj::Args;
use sui_shared_objects::consts::{
    RESULTS_DIR,
    QUERY_MAX_RESULT_LIMIT,
    SHARED_OBJECTS_SET_FILENAME,
    SHARED_OBJECTS_DATA_FILENAME,
    PACKAGES_DATA_FILENAME,
};
use sui_shared_objects::types::{
    SharedObjectsSetData,
    SharedObjectData,
    SharedObjectsData,
    ModuleAndNameData,
    PackageData,
    PackagesData,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // read the shared objects set data file
    let results_dir = Path::new(RESULTS_DIR).join(args.workspace);
    let file = fs::File::open(results_dir.join(SHARED_OBJECTS_SET_FILENAME).as_path())
        .expect("File not found!");
    let mmap = unsafe {memmap::Mmap::map(&file)}.unwrap();
    let content = std::str::from_utf8(&mmap).unwrap();
    let shared_objects_set_data: SharedObjectsSetData = serde_json::from_str(content).unwrap();

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
    let total_num_objects = shared_objects_set_data.shared_objects.len();

    // convert the set of shared object to a vector of shared object IDs
    let shared_objects_ids: Vec<ObjectID> = shared_objects_set_data.shared_objects
        .keys()
        .map(|s| ObjectID::from_str(&s).unwrap())
        .collect();

    // count the number of scanned objects
    let mut scanned_objects_count = 0;

    // bounds to slice the vector of shared object IDs
    let mut left = 0;
    let mut right = QUERY_MAX_RESULT_LIMIT;
    if total_num_objects < QUERY_MAX_RESULT_LIMIT {
        right = total_num_objects;
    }

    // If this number exceeds args.retry_number, terminate the program and save data.
    // Otherwise, sleep some time and retry query.
    let mut retry_number = 0;
    
    // repeat query is transaction or checkpoint field is None
    let mut repeat_query_on_none = false;

    // map of shared object ID to data about it
    let mut shared_objects_data = SharedObjectsData {
        total_num_shared_objects: total_num_objects,
        total_num_resources: 0,
        shared_objects: IndexMap::new()
    };

    // map of packages to data about it
    let mut packages_data = PackagesData {
        total_num_packages: 0,
        total_num_types: 0,
        total_num_resources: 0,
        packages: IndexMap::new(),
    };

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
                        print!("{}", format!("\r    Retrying query #{}/{} starting at index {} in {} s..", args.retry_number,
                            retry_number + 1, scanned_objects_count, args.retry_sleep - i).yellow());
                        std::io::stdout().flush()?;
                        sleep(Duration::from_secs(1)).await;
                    }
                    print!("{}", format!("\r    Retrying query #{}/{} starting at index {} in {} s   ", args.retry_number,
                        retry_number + 1, scanned_objects_count, 0).yellow());
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

        // process objects here
        for object in &objects {
            // Get to SuiObjectResponse {data: Some(SuiObjectData {...
            let sui_obj_data = object.data.as_ref().unwrap();

            // Get to SuiObjectData {content: Some(MoveObject(SuiParsedMoveObject {...
            if let SuiParsedData::MoveObject(sui_parsed_move_object) = sui_obj_data.content.as_ref().unwrap() {
                // temporarily variables
                let object_id = sui_obj_data
                    .object_id
                    .to_string();
                let address = sui_parsed_move_object
                    .type_
                    .address
                    .to_string();
                let is_resource = sui_parsed_move_object
                    .has_public_transfer;
                let type_ = sui_parsed_move_object
                    .type_
                    .module
                    .to_string() + &String::from(".") + &sui_parsed_move_object.type_.name.to_string();
                let num_txs = shared_objects_set_data
                    .shared_objects
                    .get(&object_id)
                    .unwrap()
                    .num_txs;
                let num_mut_refs = shared_objects_set_data
                    .shared_objects
                    .get(&object_id)
                    .unwrap()
                    .num_mut_refs;
                let first_touched_at_epoch = shared_objects_set_data
                    .shared_objects
                    .get(&object_id)
                    .unwrap()
                    .first_touched_at_epoch;
                let first_touched_at_checkpoint = shared_objects_set_data
                    .shared_objects
                    .get(&object_id)
                    .unwrap()
                    .first_touched_at_checkpoint;
                let first_touched_by_txs = &shared_objects_set_data
                    .shared_objects
                    .get(&object_id)
                    .unwrap()
                    .first_touched_by_txs;

                // update shared objects data
                shared_objects_data
                    .shared_objects
                    .insert(object_id.clone(), SharedObjectData {
                        address: address.clone(),
                        type_: type_.clone(),
                        is_resource: is_resource,
                        num_txs: num_txs,
                        num_mut_refs: num_mut_refs,
                        first_touched_at_epoch: first_touched_at_epoch,
                        first_touched_at_checkpoint: first_touched_at_checkpoint,
                        first_touched_by_txs: first_touched_by_txs.clone(),
                    });

                // update the number of resources
                if is_resource {
                    shared_objects_data.total_num_resources += 1;
                }

                // insert a new entry for package if it does not exist already
                packages_data
                    .packages
                    .entry(address.clone())
                    .or_insert(PackageData {
                        total_num_txs: 0,
                        total_num_mut_refs: 0,
                        total_num_instances: 0,
                        total_num_types: 0,
                        total_num_resources: 0,
                        types: IndexMap::new(),
                    });

                // insert a new entry for "types" if it does not exist already
                packages_data
                    .packages
                    .get_mut(&address)
                    .unwrap()
                    .types
                    .entry(type_.clone())
                    .or_insert(ModuleAndNameData {
                        num_txs: 0,
                        num_mut_refs: 0,
                        num_instances: 0,
                        is_resource: is_resource,
                        first_touched_at_epoch: first_touched_at_epoch,
                        first_touched_at_checkpoint: first_touched_at_checkpoint,
                        first_touched_by_txs: first_touched_by_txs.clone(),
                    });

                // update data for that type
                packages_data
                    .packages
                    .get_mut(&address)
                    .unwrap()
                    .types
                    .get_mut(&type_)
                    .unwrap()
                    .num_instances += 1;
                packages_data
                    .packages
                    .get_mut(&address)
                    .unwrap()
                    .types
                    .get_mut(&type_)
                    .unwrap()
                    .num_txs += num_txs;
                packages_data
                    .packages
                    .get_mut(&address)
                    .unwrap()
                    .types
                    .get_mut(&type_)
                    .unwrap()
                    .num_mut_refs += num_mut_refs;
                if first_touched_at_checkpoint <  packages_data
                        .packages
                        .get(&address)
                        .unwrap()
                        .types
                        .get(&type_)
                        .unwrap()
                        .first_touched_at_checkpoint {
                    // another shared object of that type was encountered earlier,
                    // so we need to update accordingly
                    packages_data
                        .packages
                        .get_mut(&address)
                        .unwrap()
                        .types
                        .get_mut(&type_)
                        .unwrap()
                        .first_touched_at_epoch = first_touched_at_epoch;
                    packages_data
                        .packages
                        .get_mut(&address)
                        .unwrap()
                        .types
                        .get_mut(&type_)
                        .unwrap()
                        .first_touched_at_checkpoint = first_touched_at_checkpoint;
                    packages_data
                        .packages
                        .get_mut(&address)
                        .unwrap()
                        .types
                        .get_mut(&type_)
                        .unwrap()
                        .first_touched_by_txs = first_touched_by_txs.clone();
                }
            } // end of unpacking MoveObject enum
        } // end iterating over objects

        // update count of scanned shared objects
        scanned_objects_count += objects.len();

        // update bounds
        left = right;
        right += QUERY_MAX_RESULT_LIMIT;
        if right > total_num_objects {
            right = total_num_objects;
        }
        
        print!("\rNumber of objects analyzed : {}...", format!("{}/{}", scanned_objects_count, total_num_objects).blue());
        std::io::stdout().flush()?;

        // condition to break the loop
        scanned_objects_count < total_num_objects
    } { }
    println!();

    // number of scanned objects must be equal to the number of objects in datafile
    // TODO: adapt it to handle connection error and read incomplete file
    assert_eq!(scanned_objects_count, total_num_objects);

    // The number of instances across all packages must be equal 
    // the total number of shared objects: check it.
    // Also, calculate the rest of counts
    let mut num_instances_total = 0;
    for (_, package) in packages_data.packages.iter_mut() {
        // calculate the total number of shared object types for all packages
        packages_data.total_num_types += package.types.len();

        // calculate the total number of shared object types per package
        package.total_num_types = package.types.len();

        for (_, type_) in &package.types {
            num_instances_total += type_.num_instances;

            // calculate the total number of instances per package
            package.total_num_instances += type_.num_instances;
            // calculate the total number of transactions per package
            package.total_num_txs += type_.num_txs;
            // calculate the total number of mut refs per package
            package.total_num_mut_refs += type_.num_mut_refs;

            // calculate the total number of resources
            if type_.is_resource {
                // per package
                package.total_num_resources += 1;
                // for all packages
                packages_data.total_num_resources += 1;
            }
        } // end of iterating over types

        // sort types data by num_txs in descending order
        if package.types.len() > 1 {
            let mut types_vec = Vec::from_iter(package.types.clone());
            types_vec.sort_by(|(_, a), (_, b)| b.num_txs.cmp(&a.num_txs));
            let sorted_types: IndexMap<String, ModuleAndNameData> = types_vec
                .into_iter()
                .collect();
            package.types = sorted_types;
        }
    } // end of iterating over packages
    assert_eq!(num_instances_total, total_num_objects);

    // calculate the number of packages implementing shared objects
    packages_data.total_num_packages = packages_data.packages.len();

    // sort shared objects data by num_txs in descending order
    let mut shared_objects_vec = Vec::from_iter(shared_objects_data.shared_objects);
    shared_objects_vec.sort_by(|(_, a), (_, b)| b.num_txs.cmp(&a.num_txs));
    let sorted_shared_objects: IndexMap<String, SharedObjectData> = shared_objects_vec
        .into_iter()
        .collect();
    shared_objects_data.shared_objects = sorted_shared_objects;

    // sort packages data by total_num_txs in descending order
    let mut packages_vec = Vec::from_iter(packages_data.packages);
    packages_vec.sort_by(|(_, a), (_, b)| b.total_num_txs.cmp(&a.total_num_txs));
    let sorted_packages: IndexMap<String, PackageData> = packages_vec
        .into_iter()
        .collect();
    packages_data.packages = sorted_packages;

    // save data to disk
    fs::write(
        results_dir
            .join(SHARED_OBJECTS_DATA_FILENAME),
        serde_json::to_string_pretty(&shared_objects_data)
            .unwrap()
    )?;
    fs::write(
        results_dir
            .join(PACKAGES_DATA_FILENAME),
        serde_json::to_string_pretty(&packages_data)
            .unwrap()
    )?;

    println!("{}", "Done!".green());
    Ok(())
}
