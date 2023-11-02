use clap::{Parser};

/// Estimate how often Sui transactions operate with shared objects: plot the results
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Where to store data files.
    /// This should be a directory in the "data" folder
    #[arg(short, long, default_value_t = String::from("data"))]
    pub workspace: String,

    /// Intervals in checkpoints to use for contention degree calculations
    #[arg(short, long, num_args = 1.., value_delimiter = ',',
          default_values_t = vec![1, 5, 10, 30, 60])]
    pub intervals: Vec<u64>,
}
