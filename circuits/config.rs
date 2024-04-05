use std::fmt::Debug;

pub trait TendermintConfig<const CHAIN_ID_SIZE_BYTES: usize>:
    Debug + Clone + PartialEq + Sync + Send + 'static
{
    const CHAIN_ID_BYTES: &'static [u8];
    const SKIP_MAX: usize;
}

/// @notice The maximum number of blocks that can be skipped. Below, this is set to 2 weeks, which
/// is roughly 100800 blocks if the block time is 12 seconds.
pub const SKIP_MAX: usize = 100800;

/// Celestia's chain config.
pub const CELESTIA_CHAIN_ID_BYTES: &[u8] = b"celestia";
pub const CELESTIA_CHAIN_ID_SIZE_BYTES: usize = CELESTIA_CHAIN_ID_BYTES.len();
#[derive(Debug, Clone, PartialEq)]
pub struct CelestiaConfig;
impl TendermintConfig<CELESTIA_CHAIN_ID_SIZE_BYTES> for CelestiaConfig {
    const CHAIN_ID_BYTES: &'static [u8] = CELESTIA_CHAIN_ID_BYTES;
    const SKIP_MAX: usize = SKIP_MAX;
}

/// Mocha-4's chain config.
pub const MOCHA_4_CHAIN_ID_BYTES: &[u8] = b"mocha-4";
pub const MOCHA_4_CHAIN_ID_SIZE_BYTES: usize = MOCHA_4_CHAIN_ID_BYTES.len();

/// Banksy banksy-testnet-5
pub const BANKSY_TESTNET_CHAIN_ID: &[u8] = b"banksy-testnet-5";
pub const BANKSY_TESTNET_CHAIN_ID_SIZE_BYTES: usize = BANKSY_TESTNET_CHAIN_ID.len();
#[derive(Debug, Clone, PartialEq)]
pub struct BanksyConfig;
impl TendermintConfig<BANKSY_TESTNET_CHAIN_ID_SIZE_BYTES> for BanksyConfig {
    const CHAIN_ID_BYTES: &'static [u8] = BANKSY_TESTNET_CHAIN_ID;
    const SKIP_MAX: usize = SKIP_MAX;
}
