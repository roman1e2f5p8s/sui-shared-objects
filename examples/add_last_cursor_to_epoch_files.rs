use std::fs;
use std::io::Write;
use std::path::Path;
use std::collections::BTreeMap;
use memmap;
use serde_json;
use serde::{Serialize, Deserialize};

use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::{
    TransactionFilter,
    SuiTransactionBlockResponseQuery,
};

use shared_object_density::types::{
    Epoch,
    ResultData,
    CheckpointData,
};

#[derive(Debug, Serialize, Deserialize)]
struct ResultDataOld {
    network: String,
    version: String,
    epoch: usize,
    start_checkpoint: usize,
    end_checkpoint: usize,
    num_txs_in_epoch: usize,
    num_txs_scanned: usize,
    num_txs_touching_0_shared_objs: usize,
    num_txs_touching_0_objs: usize,
    checkpoints: BTreeMap<u64, CheckpointData>
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Create a Sui client builder for connecting to the Sui network
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let results_dir = Path::new("results");
    let epoch2checkpoint_file = fs::File::open(results_dir.join("epoch2checkpoint.json"))
        .expect("File not found!");
    let epoch2checkpoint_json: BTreeMap<usize, Epoch> = 
        serde_json::from_reader(epoch2checkpoint_file)
        .expect("JSON was not properly formatted!");

    let data_dir = Path::new("data/data");
    let mut epoch_data_files: Vec<_> = fs::read_dir(data_dir).
        expect("Couldn't access directory!").
        map(|f| f.unwrap()).
        collect();
    epoch_data_files.sort_by_key(|f| f.path());

    for (i, epoch_data_file) in epoch_data_files.iter().enumerate() {
        print!("\rWorking on file {}/{}...", i + 1, epoch_data_files.len());
        let _ = std::io::stdout().flush();

        let epoch_data = fs::File::open(epoch_data_file.path())
            .expect("File not found!");
        let mmap = unsafe {memmap::Mmap::map(&epoch_data)}.unwrap();
        let content = std::str::from_utf8(&mmap).unwrap();
        let json: ResultDataOld = serde_json::from_str(content).unwrap();
        
        let checkpoint_query = SuiTransactionBlockResponseQuery::new(
            Some(TransactionFilter::Checkpoint(epoch2checkpoint_json.get(&json.epoch).unwrap().end_checkpoint as u64)), None);
        let tx_block = sui.read_api().query_transaction_blocks(
            checkpoint_query.clone(), None, Some(1), true).await?;

        let result = ResultData {
            network: json.network,
            version: json.version,
            epoch: json.epoch,
            start_checkpoint: json.start_checkpoint,
            end_checkpoint: json.end_checkpoint,
            last_cursor: tx_block.next_cursor.unwrap().to_string(),
            num_txs_in_epoch: json.num_txs_in_epoch,
            num_txs_scanned: json.num_txs_scanned,
            num_txs_touching_0_shared_objs: json.num_txs_touching_0_shared_objs,
            num_txs_touching_0_objs: json.num_txs_touching_0_objs,
            checkpoints: json.checkpoints,
        };
        
        fs::write(epoch_data_file.path(), serde_json::to_string_pretty(&result).unwrap())?;
    }
    println!();

    Ok(())
}
