use serde::{Serialize, Deserialize};
use std::collections::{
    HashSet,
    HashMap,
    BTreeMap
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TxMutInfo {
    pub tx_id: String,
    pub mutates: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckpointData {
    pub num_txs_total: usize,
    pub num_txs_touching_shared_objs: usize,
    pub shared_objects: HashMap<String, Vec<TxMutInfo>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub start_cursor: String,
    pub end_cursor: String,
    pub descending: bool,
    pub num_txs_scanned: usize,
    pub num_txs_touching_0_shared_objs: usize,
    pub num_txs_touching_0_objs: usize,
    pub checkpoints: BTreeMap<u64, CheckpointData>
}

#[derive(Debug, Deserialize)]
pub struct Epoch {
    pub start_checkpoint: usize,
    pub end_checkpoint: usize
}

#[derive(Debug, Serialize)]
pub struct EpochData {
    pub num_txs_total: usize,
    pub num_txs_touching_shared_objs: usize,
    pub density: f64,
    pub num_shared_objects: usize,
    pub num_checkpoints: usize,
    pub avg_interval_data: BTreeMap<u64, AvgIntervalData>,
}

// Counts for different checkpoint intervals 
pub struct IntervalCounts {
    pub num_txs: u64,
    pub unique_shared_objects_per_interval: HashSet<String>,
    pub unique_shared_objects_touched_by_more_than_1tx_per_interval: HashSet<String>,
}

// Data for a given interval of checkpoints
#[derive(Debug, Serialize)]
pub struct AvgIntervalData {
    pub contention_degree: f64,
    pub obj_touchability: f64,
}
