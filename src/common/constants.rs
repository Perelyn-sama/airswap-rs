//! Constants

#![allow(dead_code)]
use ethers::types::Address;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

// type Result<T, E = Box<dyn std::error::Error + Send + Sync + 'static>> = std::result::Result<T, E>;

pub const DOMAIN_NAME_SWAP_ERC20: &str = "SWAP_ERC20";
pub const DOMAIN_VERSION_SWAP_ERC20: &str = "3";
pub const DOMAIN_NAME_SWAP: &str = "SWAP";
pub const DOMAIN_VERSION_SWAP: &str = "3";
pub const DOMAIN_NAME_POOL: &str = "POOL";
pub const DOMAIN_VERSION_POOL: &str = "1";
pub const ADDRESS_ZERO: &str = "0x0000000000000000000000000000000000000000";
pub const MAX_LOCATORS: i32 = 10;
pub const MAX_APPROVAL_AMOUNT: &str = "90071992547409910000000000";
pub const MIN_CONFIRMATIONS: i32 = 2;
pub const DEFAULT_PORT: i32 = 3000;
pub const REQUEST_TIMEOUT: i32 = 4000;
pub const SECONDS_IN_DAY: i64 = 86400;

pub static MAINNET_ADDRESS: Lazy<HashMap<&str, Address>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "pool",
        Address::from_str("0xe2E7AE67E7ee6d4D90dfef945aB6dE6A14dB4c17").unwrap(),
    );
    hash_map.insert(
        "registry",
        Address::from_str("0x8F9DA6d38939411340b19401E8c54Ea1f51B8f95").unwrap(),
    );
    hash_map.insert(
        "staking",
        Address::from_str("0x6d88B09805b90dad911E5C5A512eEDd984D6860B").unwrap(),
    );
    hash_map.insert(
        "swap_erc20",
        Address::from_str("0xb1B586AfA8a2AaB42826Fb2Ab9896CD0c686d0F4").unwrap(),
    );
    hash_map.insert(
        "wrapper",
        Address::from_str("0x5E5A433cdfB14aB228c45E23251Ad83F7b1E3302").unwrap(),
    );
    hash_map
});

pub static CHAIN_IDS: Lazy<HashMap<&str, usize>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert("ETHEREUM", 1);
    hash_map.insert("GOERLI", 5);
    hash_map.insert("OPTIMISM", 10);
    hash_map.insert("RSK", 30);
    hash_map.insert("RSKTESTNET", 31);
    hash_map.insert("KOVAN", 42);
    hash_map.insert("BSC", 56);
    hash_map.insert("BSCTESTNET", 137);
    hash_map.insert("ARBITRUM", 42161);
    hash_map.insert("ARBITRUMGOERLI", 421613);
    hash_map.insert("AVALANCHE", 43114);
    hash_map.insert("MUMBAI", 80001);
    hash_map
});

pub static CHAIN_NAMES: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, "ETHEREUM");
    hash_map.insert(5, "GOERLI");
    hash_map.insert(10, "OPTIMISM");
    hash_map.insert(30, "RSK");
    hash_map.insert(31, "RSKTESTNET");
    hash_map.insert(42, "KOVAN");
    hash_map.insert(56, "BSC");
    hash_map.insert(97, "BSCTESTNET");
    hash_map.insert(137, "POLYGON");
    hash_map.insert(42161, "ARBITRUM");
    hash_map.insert(43113, "FUJI");
    hash_map.insert(43114, "AVALANCHE");
    hash_map.insert(80001, "MUMBAI");
    hash_map.insert(421613, "ARBITRUMGOERLI");
    hash_map
});

pub const MAINNETS: [i32; 6] = [1, 30, 56, 137, 42161, 43114];

pub const TESTNETS: [i32; 6] = [5, 31, 97, 80001, 421613, 43113];

static CHAIN_CURRENCIES: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, "ETH");
    hash_map.insert(5, "ETH");
    hash_map.insert(30, "RBTC");
    hash_map.insert(31, "tRBTC");
    hash_map.insert(42, "ETH");
    hash_map.insert(56, "BNB");
    hash_map.insert(97, "BNB");
    hash_map.insert(137, "MATIC");
    hash_map.insert(42161, "AETH");
    hash_map.insert(421613, "AETH");
    hash_map.insert(43113, "AVAX");
    hash_map.insert(43114, "AVAX");
    hash_map.insert(80001, "MATIC");
    hash_map
});

static WRAPPED_TOKEN_ADDRESSES: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    hash_map.insert(5, "0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6");
    hash_map.insert(30, "0x967f8799af07df1534d48a95a5c9febe92c53ae0");
    hash_map.insert(31, "0x09b6ca5e4496238a1f176aea6bb607db96c2286e");
    hash_map.insert(42, "0xd0a1e359811322d97991e03f863a0c30c2cf029c");
    hash_map.insert(56, "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c");
    hash_map.insert(97, "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd");
    hash_map.insert(137, "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270");
    hash_map.insert(42161, "0x82af49447d8a07e3bd95bd0d56f35241523fbab1");
    hash_map.insert(421613, "0xd9d01a9f7c810ec035c0e42cb9e80ef44d7f8692");
    hash_map.insert(43113, "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7");
    hash_map.insert(43114, "0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889");
    hash_map.insert(80001, "0xee01c0cd76354c383b8c7b4e65ea88d00b06f36f");
    hash_map
});

static STAKING_TOKEN_ADDRESSES: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, "0x27054b13b1b798b345b591a4d22e6562d47ea75a");
    hash_map.insert(5, "0x1a1ec25dc08e98e5e93f1104b5e5cdd298707d31");
    hash_map.insert(30, "");
    hash_map.insert(31, "0x9c7005fa2f8476e2331f45f69e0930a4c9eff0c3");
    hash_map.insert(42, "0x1a1ec25dc08e98e5e93f1104b5e5cdd298707d31");
    hash_map.insert(56, "0x1ac0d76f11875317f8a7d791db94cdd82bd02bd1");
    hash_map.insert(97, "0xd161ddcfcc0c2d823021aa26200824efa75218d1");
    hash_map.insert(137, "0x04bEa9FCE76943E90520489cCAb84E84C0198E29");
    hash_map.insert(42161, "0xa1135c2f2c7798d31459b5fdaef8613419be1008");
    hash_map.insert(43113, "0x48c427e7cEf42399e9e8300fC47875772309e995");
    hash_map.insert(43114, "0x702d0f43edd46b77ea2d48570b02c328a20a94a1");
    hash_map.insert(80001, "0xd161ddcfcc0c2d823021aa26200824efa75218d1");
    hash_map.insert(421613, "0x71070c5607358fc25e3b4aaf4fb0a580c190252a");
    hash_map
});

static ETHERSCAN_DOMAINS: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, "etherscan.io");
    hash_map.insert(5, "goerli.etherscan.io");
    hash_map.insert(30, "explorer.rsk.co");
    hash_map.insert(31, "explorer.testnet.rsk.co");
    hash_map.insert(42, "kovan.etherscan.io");
    hash_map.insert(56, "bscscan.com");
    hash_map.insert(97, "testnet.bscscan.com");
    hash_map.insert(137, "polygonscan.com");
    hash_map.insert(42161, "arbiscan.io");
    hash_map.insert(43113, "testnet.snowtrace.io");
    hash_map.insert(43114, "snowtrace.io");
    hash_map.insert(80001, "mumbai.polygonscan.com");
    hash_map.insert(421613, "goerli.arbiscan.io");
    hash_map
});

#[derive(Debug)]
pub enum ItemType {
    ERC20 = 0x36372b07,
    ERC721 = 0x80ac58cd,
    ERC1155 = 0xd9b67a26,
    CKITTY = 0x9a20483d,
}

// export const tokenKinds = {
//   ERC20: TokenKinds.ERC20,
//   ERC721: TokenKinds.ERC721,
//   ERC1155: TokenKinds.ERC1155,
//   CKITTY: TokenKinds.CKITTY,
// }

// export const tokenKindNames: Record<string, string> = {
//   '0x36372b07': 'ERC20',
//   '0x80ac58cd': 'ERC721',
//   '0xd9b67a26': 'ERC1155',
//   '0x9a20483d': 'CKITTY',
// }
