use clap::{Parser, ValueEnum};
use strum_macros::Display;

/// Estimate how often Sui transactions operate with shared objects: query TXs
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Which network to use
    #[arg(short, long, value_enum, default_value_t = NetworkType::Mainnet)]
    pub network: NetworkType,

    /// Epoch to scan all TXs from it >= 0
    #[arg(short, long)]
    pub epoch: usize,

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

#[derive(ValueEnum, Debug, Clone, Display)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
}
