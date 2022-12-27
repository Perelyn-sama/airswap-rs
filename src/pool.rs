use crate::bindings::pool::pool::pool;
use ethers::prelude::{builders::ContractCall, *};

#[derive(Debug)]
pub struct Pool<M> {
    /// The Pool Airswap contract.
    pub contract: pool<M>,
}

impl<M> Pool<M> {
    /// Returns a reference to the Airswap Pool contract.
    pub fn contract(&self) -> &pool<M> {
        &self.contract
    }
}

impl<M: Middleware> Seaport<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = seaport::new(address, client);
        Self {contract}
    }

    /// Add admin address
    /// Only owner
    pub fn add_admin(&self, _admin: Address) -> ContractCall<M,()> {
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
}

impl<M> std::ops::Deref for Pool<M> {
    type Target = pool<M>;
    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}
