use crate::contracts::bindings::i_pool::i_pool::IPool;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;
#[derive(Debug)]
pub struct Pool<M> {
    /// The Pool Airswap contract.
    pub contract: IPool<M>,
}

impl<M> Pool<M> {
    /// Returns a reference to the Airswap Pool contract.
    pub fn contract(&self) -> &IPool<M> {
        &self.contract
    }
}

impl<M: Middleware> Pool<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IPool::new(address, client);
        Self { contract }
    }

    /// Add admin address
    /// Only owner
    pub fn add_admin(&self, _admin: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.add_admin(_admin)
    }

    /// Admin function to migrate funds
    /// Only owner
    pub fn drain_to(&self, tokens: Vec<Address>, dest: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.drain_to(tokens, dest)
    }

    /// Remove admin address
    /// Only owner
    pub fn remove_admin(&self, _admin: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.remove_admin(_admin)
    }

    /// Set max
    /// Only owner
    pub fn set_max(&self, _max: U256) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.set_max(_max)
    }
    /// Set scale
    /// Only owner
    pub fn set_scale(&self, _scale: U256) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.set_scale(_scale)
    }

    /// Set staking contract address
    /// Only owner
    pub fn set_staking_contract(&self, staking_contract: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.set_staking_contract(staking_contract)
    }

    /// Set staking token address
    /// Only owner
    pub fn set_staking_token(&self, staking_token: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.set_staking_token(staking_token)
    }

    /// Transfers ownership of the contract to a new account (`newOwner`). Can only be called by the current owner.
    pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<M, ()> {
        let pool = self.contract();
        pool.transfer_ownership(new_owner)
    }

    /// Withdraw tokens from the pool using a signed claim
    pub fn withdraw(
        &self,
        recipient: Address,
        minimum: U256,
        token: Address,
        nonce: U256,
        expiry: U256,
        score: U256,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> ContractCall<M, U256> {
        let pool = self.contract();
        pool.withdraw(recipient, minimum, token, nonce, expiry, score, v, r, s)
    }

    /// Withdraw tokens from the pool using signature and stake for a recipient
    pub fn withdraw_and_stake(
        &self,
        recipient: Address,
        minimum: U256,
        token: Address,
        nonce: U256,
        expiry: U256,
        score: U256,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> ContractCall<M, U256> {
        let pool = self.contract();
        pool.withdraw_and_stake(recipient, minimum, token, nonce, expiry, score, v, r, s)
    }
}

impl<M> std::ops::Deref for Pool<M> {
    type Target = IPool<M>;
    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}
