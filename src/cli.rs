use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(
    about = "Pyth Tool - the admin swiss army knife",
    author = "Pyth Network Contributors"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
    /// Mainnet/testnet
    #[clap(default_value = "testnet")]
    pub net: Net,
}

#[derive(Subcommand)]
pub enum Action {
    #[clap(about = "Sanity-check access to all known blockchains")]
    PingAll,
}

/// For most chains, we pick a production blockchain network and a
/// testing one, usually closely following Wormhole's  choices.
#[derive(ValueEnum, Clone)]
pub enum Net {
    Mainnet,
    Testnet,
}
