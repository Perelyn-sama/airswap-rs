//! # airswap-rs
//!
//! Unofficial Rust SDK library for Airswap smart contracts.

pub mod common;
pub mod contracts;
pub mod pool;
pub mod registry;
pub mod staking;
pub mod swap_erc20;
pub mod utils;
pub mod wrapper;

pub mod prelude {
    pub use super::{
        common::constants::ADDRESS_ZERO,
        pool::Pool,
        registry::Registry,
        staking::Staking,
        swap_erc20::SwapERC20,
        utils::{pool::create_claim, swap::create_order},
        wrapper::Wrapper,
    };

    use ethers::{
        abi::Detokenize,
        prelude::{builders::ContractCall, *},
    };
    use eyre::ContextCompat;

    pub async fn send<M: Middleware + 'static, D: Detokenize>(
        call: ContractCall<M, D>,
    ) -> eyre::Result<TransactionReceipt> {
        let pending_tx = call.send().await?;
        println!("Transaction sent successfully, awaiting inclusion...");
        pending_tx
            .await?
            .wrap_err("transaction was dropped from mempool")
    }

    #[cfg(feature = "addresses")]
    pub use super::contracts::addresses::{address, contract, try_address, try_contract};
}
