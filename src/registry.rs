use std::sync::Arc;
use ethers::prelude::{builders::ContractCall, *};
use crate::bindings::{i_registry::i_registry::IRegistry};

#[derive(Debug)]
pub struct Registry<M> {
    /// The Registry Airswap contract.
    pub contract: IRegistry<M>
}

impl<M> Registry<M> {
    /// Returns a reference to the Airswap Regisry contract.
    pub fn contract(&self) -> &IRegistry<M> {
        &self.contract
    }
}

impl<M: Middleware> Registry<M>{
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self{
        let contract = IRegistry::new(address, client);
        Self {contract}
    }

    /// Add tokens supported by the caller
    pub fn add_tokens(&self, tokens: Vec<Address>) -> ContractCall<M, ()>{
        let registry = self.contract();
        registry.add_tokens(tokens)
    }

    /// Remove all tokens supported by the caller
    pub fn remove_all_tokens(&self, tokens: Vec<Address>) -> ContractCall<M, ()>{
        let registry = self.contract();
        registry.remove_all_tokens()
    }

    /// Remove tokens supported by the caller
    pub fn remove_tokens(&self, tokens: Vec<Address>) -> ContractCall<M, ()>{
        let registry = self.contract();
        registry.remove_tokens(tokens)
    }
    /// Set the URL for a staker
    pub fn set_url(&self, _url: String) -> ContractCall<M, ()>{
        let registry = self.contract();
        registry.set_url(_url)
    }
}