use crate::contracts::bindings::i_wrapper::i_wrapper::IWrapper;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

#[derive(Debug)]
pub struct Wrapper<M> {
    /// The Wrapper Airswap contract.
    pub contract: IWrapper<M>,
}

impl<M> Wrapper<M> {
    /// Returns a reference to the Airswap Wrapper contract.
    pub fn contract(&self) -> &IWrapper<M> {
        &self.contract
    }
}

impl<M: Middleware> Wrapper<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IWrapper::new(address, client);
        Self { contract }
    }

    /// Set the swap contract
    pub fn set_swap_contract(&self, swap_contract: Address) -> ContractCall<M, ()> {
        let wrapper = self.contract();
        wrapper.set_swap_contract(swap_contract)
    }

    /// Wrapped SwapERC20.swap
    pub fn swap(
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
        let wrapper = self.contract();
        wrapper.swap(
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

    /// Wrapped SwapERC20.swapAnySender
    pub fn swap_any_sender(
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
        let wrapper = self.contract();
        wrapper.swap_any_sender(
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

    /// Transfers ownership of the contract to a new account (`newOwner`). Can only be called by the current owner.
    pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<M, ()> {
        let wrapper = self.contract();
        wrapper.transfer_ownership(new_owner)
    }
}
