use clap::{
    Parser,
    ValueEnum
};
use strum_macros::Display;

/// Query all the transactions (i.e., programmable transaction blocks) for a 
/// given epoch, and pre-process them to save only the relevant data we need 
/// for this analysis.
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Epoch to scan all TXs from, >= 0
    #[arg(short, long)]
    pub epoch: usize,

    /// Which network to use
    #[arg(short, long, value_enum, default_value_t = NetworkType::Mainnet)]
    pub network: NetworkType,

    /// Where to store data files. This should be a directory in the 
    /// "data" folder
    #[arg(short, long, default_value_t = String::from("workspace1"))]
    pub workspace: String,

    /// Number of query retries, >= 0
    #[arg(short, long, default_value_t = 10)]
    pub retry_number: usize,

    /// Sleep time between retries in whole seconds, >= 0
    #[arg(short = 's', long, default_value_t = 10)]
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
