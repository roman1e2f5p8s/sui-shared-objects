use clap::{Parser, ValueEnum};
use strum_macros::Display;

/// Sui shared objects analysis: query shared objects
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Where to store result files.
    /// This should be a directory in the "result" folder
    #[arg(short, long, default_value_t = String::from("workspace1"))]
    pub workspace: String,

    /// Which network to use
    #[arg(short, long, value_enum, default_value_t = NetworkType::Mainnet)]
    pub network: NetworkType,

    /// Number of query retries, >= 0
    #[arg(short, long, default_value_t = 10)]
    pub retry_number: usize,

    /// Sleep time between reties in whole seconds, >= 0
    #[arg(short, long, default_value_t = 10)]
    pub retry_sleep: u64,

    /// Print detailed output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

/// Sui network type: mainnet, testnet, devnet
#[derive(ValueEnum, Debug, Clone, Display)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
}
