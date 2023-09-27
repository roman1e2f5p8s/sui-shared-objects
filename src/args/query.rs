use clap::{Parser, ValueEnum};

/// Estimate how often Sui transactions operate with shared objects: query TXs
#[derive(Parser, Debug)]
#[command(author = "Roman Overko", version, about, long_about = None)]
pub struct Args {
    /// Which network to use
    #[arg(short, long, value_enum, default_value_t = NetworkType::Mainnet)]
    pub network: NetworkType,

    /// Number of TXs to scan, >= 0
    #[arg(short, long, default_value_t = 1000)]
    pub tx_number: usize,

    /// Digest of TX from which to start scanning.
    /// The corresponding TX won't be scaned!
    /// If empty: if --descending, scans the latest TXs;
    /// otherwise, scans the first TXs
    #[arg(short, long, default_value_t = String::from(""))]
    pub cursor: String,

    /// Scan TXs in descending order
    #[arg(short, long, default_value_t = false)]
    pub descending: bool,

    /// Print detailed output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Devnet,
}