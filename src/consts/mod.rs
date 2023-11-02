// Keeping all the constants here to avoid "magic number"

// Maximum number of results returned by query.
// This value is taken from
// https://github.com/MystenLabs/sui/blob/main/crates/sui-json-rpc/src/api/mod.rs
// and must be updated if it is changed there.
pub const QUERY_MAX_RESULT_LIMIT: usize = 50;

// Name of directory where the results (figures) are stored
pub const RESULTS_DIR: &str = "results";

// Name of file where shared objects ID are stored as strings
pub const SHARED_OBJECTS_SET_FILENAME: &str = "shared_objects_set.json";
