use clap::Parser;

/// Calculate metrics used to analyze shared objects and obtain a list of
/// all shared object IDs for further analysis
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Where to store data files. This should be a directory in the "results" 
    /// folder with the same name as used in `query-txs`
    #[arg(short, long, default_value_t = String::from("workspace1"))]
    pub workspace: String,

    /// Intervals (in checkpoints) to use for contention degree calculations
    #[arg(short, long, num_args = 1.., value_delimiter = ',',
          default_values_t = vec![1, 5, 10, 30, 60])]
    pub intervals: Vec<u64>,

    /// Print detailed output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
