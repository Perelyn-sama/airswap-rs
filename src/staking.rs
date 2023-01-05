use crate::bindings::i_staking::i_staking::IStaking;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;

#[derive(Debug)]
pub struct Staking<M> {
    /// The Staking Airswap contract.
    pub contract: IStaking<M>,
}

impl<M> Staking<M> {
    /// Returns a reference to the Airswap Staking contract.
    pub fn contract(&self) -> &IStaking<M> {
        &self.contract
    }
}

impl<M: Middleware> Staking<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IStaking::new(address, client);
        Self { contract }
    }

    /// Cancels timelock to change duration
    pub fn cancel_duration_change(&self) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.cancel_duration_change()
    }

    /// Propose delegate for account
    pub fn propose_delegate(&self, delegate: Address) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.propose_delegate(delegate)
    }

    /// Leaves the contract without owner. It will not be possible to call `onlyOwner` functions anymore. Can only be called by the current owner. NOTE: Renouncing ownership will leave the contract without an owner, thereby removing any functionality that is only available to the owner.
    pub fn renounce_ownership(&self) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.renounce_ownership()
    }

    /// Schedules timelock to change duration
    pub fn schedule_duration_change(&self, delay: U256) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.schedule_duration_change(delay)
    }

    /// Set delegate for account
    pub fn set_delegate(&self, account: Address) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.set_delegate(account)
    }

    /// Set unstaking duration
    pub fn set_duration(&self, _duration: U256) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.set_duration(_duration)
    }

    /// Set metadata config
    pub fn set_meta_data(&self, _name: String, _symbol: String) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.set_meta_data(_name, _symbol)
    }

    /// Stake tokens
    pub fn stake(&self, amount: U256) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.stake(amount)
    }

    /// Stake tokens for an account
    pub fn stake_for(&self, account: Address, amount: U256) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.stake_for(account, amount)
    }

    /// Transfers ownership of the contract to a new account (`newOwner`). Can only be called by the current owner.
    pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.transfer_ownership(new_owner)
    }

    /// Unset delegate for account
    pub fn unset_delegate(&self, delegate: Address) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.unset_delegate(delegate)
    }

    /// Unstake tokens
    pub fn unstake(&self, amount: U256) -> ContractCall<M, ()> {
        let staking = self.contract();
        staking.unstake(amount)
    }
}
