// Keeping all the constants here to avoid "magic number"

// Maximum number of results returned by query.
// This value is taken from
// https://github.com/MystenLabs/sui/blob/main/crates/sui-json-rpc/src/api/mod.rs
// and must be updated if it is changed there.
pub const QUERY_MAX_RESULT_LIMIT: usize = 50;

// Name of directory where the pre-processed data is stored
pub const DATA_DIR: &str = "data";

// Name of directory where the results (figures) are stored
pub const RESULTS_DIR: &str = "results";

// file that contains a mapping from epoch to checkpoints
pub const EPOCH_TO_CHECKPOINTS_FILENAME: &str = "EPOCH_TO_CHECKPOINTS.json";

// Name of file where epochal density data is stored
pub const PLOT_FILENAME: &str = "epoch_density_data.json";

// Name of file where shared objects ID are stored as strings
pub const SHARED_OBJECTS_SET_FILENAME: &str = "shared_objects_set.json";

// Name of file where shared objects data is stored
pub const SHARED_OBJECTS_DATA_FILENAME: &str = "shared_objects_data.json";

// Name of file where packages data is stored
pub const PACKAGES_DATA_FILENAME: &str = "packages_data.json";
