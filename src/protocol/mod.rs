//! Top-level module for protocol-specific behavior. Abstraction hierarchy:
//! Protocol has many Chains (differentiated by mainnet/testnet), Chain has many RPCConfigs

use util::ErrBox;

pub trait Protocol<const NET: Net> {
    async fn ping() -> Result<(), ErrBox>;
}

/// For most chains, we pick a production blockchain network and a
/// testing one, usually closely following Wormhole's  choices.
#[derive(ValueEnum, Clone)]
pub enum Net {
    Mainnet,
    Testnet,
}
