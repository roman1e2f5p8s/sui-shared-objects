use serde::{Serialize, Deserialize};
use std::collections::{
    HashSet,
    BTreeMap
};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct SharedObjInfo {
    pub id: String,
    pub mutable: bool
}

#[derive(Debug)]
pub struct TxInfo {
    pub num_total: usize,
    pub num_shared: usize,
    pub shared_objects: Vec<SharedObjInfo>
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TxMutInfo {
//     pub tx_id: String,
//     pub mutates: bool
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckpointData {
    pub num_txs_total: usize,
    pub num_txs_touching_shared_objs: usize,
    pub shared_objects: BTreeMap<
        String,     // object ID
        BTreeMap<
            String, // TX ID
            bool,   // whether this TX mutates obj or not
            >
        >
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub network: String,
    pub version: String,
    pub epoch: usize,
    pub start_checkpoint: usize,
    pub end_checkpoint: usize,
    pub last_cursor: String,
    pub num_txs_in_epoch: usize,
    pub num_txs_scanned: usize,
    pub num_txs_touching_0_shared_objs: usize,
    pub num_txs_touching_0_objs: usize,
    pub checkpoints: BTreeMap<u64, CheckpointData>
}

#[derive(Debug, Deserialize)]
pub struct Epoch {
    pub start_checkpoint: usize,
    pub end_checkpoint: usize,
    pub tx_number: usize
}

// defines epoch-to-checkpoint data file structure
#[derive(Debug, Deserialize)]
pub struct EpochToCheckpointData {
    pub epochs: BTreeMap<usize, Epoch>,
}

// contains data about a single epoch
#[derive(Debug, Serialize)]
pub struct EpochData {
    pub num_txs_total: usize,
    pub num_txs_touching_shared_objs: usize,
    pub density: f64,
    pub num_shared_objects_per_epoch: usize,
    pub num_shared_objects_total: usize,
    pub num_checkpoints: usize,
    pub avg_interval_data: BTreeMap<u64, AvgIntervalData>,
}

// Counts for different checkpoint intervals 
#[derive(Debug)]
pub struct IntervalCounts {
    pub num_txs: u64,
    pub unique_shared_objects: HashSet<String>,
    pub unique_shared_objects_touched_by_more_than_1tx: HashSet<String>,
}

// Data for a given interval of checkpoints
#[derive(Debug, Serialize)]
pub struct AvgIntervalData {
    pub contention_degree: f64,
    pub obj_touchability: f64,
}

// stores data about all epochs
#[derive(Debug, Serialize)]
pub struct EpochsData {
    pub epochs: BTreeMap<usize, EpochData>,
}

// stores data of interest about a shared object
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedObjectSetData {
    pub num_txs: usize,
    pub num_mut_refs: usize,
    pub first_touched_at_epoch: usize,
    pub first_touched_at_checkpoint: u64,
    pub first_touched_by_txs: BTreeMap<String, bool>,
}

// stores data of interest about the set of all shared objects
#[derive(Debug, Serialize, Deserialize)]
pub struct SharedObjectsSetData {
    pub shared_objects: BTreeMap<String, SharedObjectSetData>,
}

// stores more data of interest about a single shared object 
#[derive(Debug, Serialize)]
pub struct SharedObjectData {
    pub address: String,
    pub type_: String,
    pub is_resource: bool,
    pub num_txs: usize,
    pub num_mut_refs: usize,
    pub first_touched_at_epoch: usize,
    pub first_touched_at_checkpoint: u64,
    pub first_touched_by_txs: BTreeMap<String, bool>,
}

// stores data of interest about all shared objects
#[derive(Debug, Serialize)]
pub struct SharedObjectsData {
    pub total_num_shared_objects: usize,
    pub total_num_resources: usize,
    pub shared_objects: IndexMap<String, SharedObjectData>,
}

// stores data of interest about shared objects
// based on their type (i.e., module and name)
#[derive(Debug, Clone, Serialize)]
pub struct ModuleAndNameData {
    pub num_txs: usize,
    pub num_mut_refs: usize,
    pub num_instances: usize,
    pub is_resource: bool,
    pub first_touched_at_epoch: usize,
    pub first_touched_at_checkpoint: u64,
    pub first_touched_by_txs: BTreeMap<String, bool>,
}

// stores data of interest about a package
#[derive(Debug, Serialize)]
pub struct PackageData {
    pub total_num_txs: usize,
    pub total_num_mut_refs: usize,
    pub total_num_instances: usize,
    pub total_num_types: usize,
    pub total_num_resources: usize,
    pub types: IndexMap<String, ModuleAndNameData>,
}

// stores data of interest about all packages
#[derive(Debug, Serialize)]
pub struct PackagesData {
    pub total_num_packages: usize,
    pub total_num_types: usize,
    pub total_num_resources: usize,
    pub packages: IndexMap<String, PackageData>,
}
