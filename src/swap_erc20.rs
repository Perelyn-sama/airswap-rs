use crate::bindings::i_swap_erc20::i_swap_erc20::ISwapERC20;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;
#[derive(Debug)]
pub struct SwapERC20<M> {
    /// The SwapERC20 contract
    pub contract: ISwapERC20<M>,
}

impl<M> SwapERC20<M> {
    /// Returns a reference to the SwapERC20
    pub fn contract(&self) -> &ISwapERC20<M> {
        &self.contract
    }
}

impl<M: Middleware> SwapERC20<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = ISwapERC20::new(address, client);
        Self { contract }
    }

    /// Authorize a signer
    /// Emits an Authorize event
    pub fn authorize(&self, signer: Address) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.authorize(signer)
    }

    /// Cancel one or more nonces
    /// Cancelled nonces are marked as usedEmits a Cancel eventOut of gas may occur in arrays of length > 400
    pub fn cancel(&self, nonces: Vec<U256>) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.cancel(nonces)
    }

    /// Revoke the signer
    /// Emits a Revoke event
    pub fn revoke(&self) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.revoke()
    }

    /// Set the fee
    pub fn set_protocol_fee(&self, protocol_fee: U256) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_protocol_fee(protocol_fee)
    }

    /// Set the light fee
    pub fn set_protocol_light_fee(&self, protocol_fee_light: U256) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_protocol_fee_light(protocol_fee_light)
    }

    /// Set the fee wallet
    pub fn set_protocol_fee_wallet(&self, protocol_fee_wallet: Address) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_protocol_fee_wallet(protocol_fee_wallet)
    }

    /// Set max
    /// Only owner
    pub fn set_rebate_max(&self, rebate_max: U256) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_rebate_max(rebate_max)
    }

    /// Set scale
    /// Only owner
    pub fn set_rebate_scale(&self, rebate_scale: U256) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_rebate_scale(rebate_scale)
    }

    /// Set the staking token
    pub fn set_staking(&self, new_staking: Address) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.set_staking(new_staking)
    }

    /// Atomic ERC20 Swap
    pub fn swap(
        &self,
        recipient: Address,
        nonce: U256,
        expiry: U256,
        signer_wallet: Address,
        signer_token: Address,
        signer_amount: U256,
        sender_token: Address,
        sender_amount: U256,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.swap(
            recipient,
            nonce,
            expiry,
            signer_wallet,
            signer_token,
            signer_amount,
            sender_token,
            sender_amount,
            v,
            r,
            s,
        )
    }

    /// Atomic ERC20 Swap for Any Sender
    pub fn swap_any_sender(
        &self,
        recipient: Address,
        nonce: U256,
        expiry: U256,
        signer_wallet: Address,
        signer_token: Address,
        signer_amount: U256,
        sender_token: Address,
        sender_amount: U256,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.swap_any_sender(
            recipient,
            nonce,
            expiry,
            signer_wallet,
            signer_token,
            signer_amount,
            sender_token,
            sender_amount,
            v,
            r,
            s,
        )
    }

    /// Swap Atomic ERC20 Swap (Low Gas Usage)
    pub fn swap_light(
        &self,
        nonce: U256,
        expiry: U256,
        signer_wallet: Address,
        signer_token: Address,
        signer_amount: U256,
        sender_token: Address,
        sender_amount: U256,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> ContractCall<M, ()> {
        let swap_erc20 = self.contract();
        swap_erc20.swap_light(
            nonce,
            expiry,
            signer_wallet,
            signer_token,
            signer_amount,
            sender_token,
            sender_amount,
            v,
            r,
            s,
        )
    }
}
