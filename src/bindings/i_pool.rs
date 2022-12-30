pub use i_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_pool {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scale\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_max\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_stakingContract\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_stakingToken\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"dest\",\"type\":\"address\"}],\"name\":\"DrainTo\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"max\",\"type\":\"uint256\"}],\"name\":\"SetMax\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"scale\",\"type\":\"uint256\"}],\"name\":\"SetScale\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"score\",\"type\":\"uint256\"}],\"name\":\"Withdraw\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"CLAIM_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_CHAIN_ID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_NAME\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_VERSION\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\"}],\"name\":\"addAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"admins\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"score\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"calculate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"},{\"internalType\":\"address\",\"name\":\"dest\",\"type\":\"address\"}],\"name\":\"drainTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"max\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"participant\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"name\":\"nonceUsed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\"}],\"name\":\"removeAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"scale\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_max\",\"type\":\"uint256\"}],\"name\":\"setMax\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_scale\",\"type\":\"uint256\"}],\"name\":\"setScale\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_stakingContract\",\"type\":\"address\"}],\"name\":\"setStakingContract\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_stakingToken\",\"type\":\"address\"}],\"name\":\"setStakingToken\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"stakingContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"stakingToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"participant\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"score\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"verify\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"minimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"score\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"minimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"score\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"withdrawAndStake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IPool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPool<M> {
        fn clone(&self) -> Self {
            IPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOL_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `CLAIM_TYPEHASH` (0x6b0509b1) function"]
        pub fn claim_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 5, 9, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_CHAIN_ID` (0x416f281d) function"]
        pub fn domain_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 111, 40, 29], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_NAME` (0x796f077b) function"]
        pub fn domain_name(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 111, 7, 123], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function"]
        pub fn domain_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_VERSION` (0xacb8cc49) function"]
        pub fn domain_version(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([172, 184, 204, 73], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addAdmin` (0x70480275) function"]
        pub fn add_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 72, 2, 117], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admins` (0x429b62e5) function"]
        pub fn admins(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 155, 98, 229], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculate` (0x5a1d249d) function"]
        pub fn calculate(
            &self,
            score: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 29, 36, 157], (score, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drainTo` (0xb09013a2) function"]
        pub fn drain_to(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            dest: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 144, 19, 162], (tokens, dest))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `max` (0x6ac5db19) function"]
        pub fn max(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 197, 219, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonceUsed` (0x1647795e) function"]
        pub fn nonce_used(
            &self,
            participant: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([22, 71, 121, 94], (participant, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeAdmin` (0x1785f53c) function"]
        pub fn remove_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 133, 245, 60], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scale` (0xf51e181a) function"]
        pub fn scale(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 30, 24, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMax` (0x1fe9eabc) function"]
        pub fn set_max(
            &self,
            max: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 233, 234, 188], max)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setScale` (0x3edc3519) function"]
        pub fn set_scale(
            &self,
            scale: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 220, 53, 25], scale)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStakingContract` (0x9dd373b9) function"]
        pub fn set_staking_contract(
            &self,
            staking_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 211, 115, 185], staking_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStakingToken` (0x1e9b12ef) function"]
        pub fn set_staking_token(
            &self,
            staking_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 155, 18, 239], staking_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakingContract` (0xee99205c) function"]
        pub fn staking_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([238, 153, 32, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakingToken` (0x72f702f3) function"]
        pub fn staking_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([114, 247, 2, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verify` (0x2e0f5429) function"]
        pub fn verify(
            &self,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            participant: ethers::core::types::Address,
            score: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [46, 15, 84, 41],
                    (nonce, expiry, participant, score, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xf7c828aa) function"]
        pub fn withdraw(
            &self,
            recipient: ethers::core::types::Address,
            minimum: ethers::core::types::U256,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            score: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [247, 200, 40, 170],
                    (recipient, minimum, token, nonce, expiry, score, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndStake` (0x8df32af5) function"]
        pub fn withdraw_and_stake(
            &self,
            recipient: ethers::core::types::Address,
            minimum: ethers::core::types::U256,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            score: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [141, 243, 42, 245],
                    (recipient, minimum, token, nonce, expiry, score, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DrainTo` event"]
        pub fn drain_to_filter(&self) -> ethers::contract::builders::Event<M, DrainToFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetMax` event"]
        pub fn set_max_filter(&self) -> ethers::contract::builders::Event<M, SetMaxFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetScale` event"]
        pub fn set_scale_filter(&self) -> ethers::contract::builders::Event<M, SetScaleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPoolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPool<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DrainTo", abi = "DrainTo(address[],address)")]
    pub struct DrainToFilter {
        pub tokens: Vec<ethers::core::types::Address>,
        pub dest: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetMax", abi = "SetMax(uint256)")]
    pub struct SetMaxFilter {
        pub max: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetScale", abi = "SetScale(uint256)")]
    pub struct SetScaleFilter {
        pub scale: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "Withdraw",
        abi = "Withdraw(uint256,uint256,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub nonce: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub expiry: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub score: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolEvents {
        DrainToFilter(DrainToFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SetMaxFilter(SetMaxFilter),
        SetScaleFilter(SetScaleFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DrainToFilter::decode_log(log) {
                return Ok(IPoolEvents::DrainToFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(IPoolEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SetMaxFilter::decode_log(log) {
                return Ok(IPoolEvents::SetMaxFilter(decoded));
            }
            if let Ok(decoded) = SetScaleFilter::decode_log(log) {
                return Ok(IPoolEvents::SetScaleFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(IPoolEvents::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolEvents::DrainToFilter(element) => element.fmt(f),
                IPoolEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                IPoolEvents::SetMaxFilter(element) => element.fmt(f),
                IPoolEvents::SetScaleFilter(element) => element.fmt(f),
                IPoolEvents::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `CLAIM_TYPEHASH` function with signature `CLAIM_TYPEHASH()` and selector `[107, 5, 9, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "CLAIM_TYPEHASH", abi = "CLAIM_TYPEHASH()")]
    pub struct ClaimTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_CHAIN_ID` function with signature `DOMAIN_CHAIN_ID()` and selector `[65, 111, 40, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_CHAIN_ID", abi = "DOMAIN_CHAIN_ID()")]
    pub struct DomainChainIdCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_NAME` function with signature `DOMAIN_NAME()` and selector `[121, 111, 7, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_NAME", abi = "DOMAIN_NAME()")]
    pub struct DomainNameCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_VERSION` function with signature `DOMAIN_VERSION()` and selector `[172, 184, 204, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_VERSION", abi = "DOMAIN_VERSION()")]
    pub struct DomainVersionCall;
    #[doc = "Container type for all input parameters for the `addAdmin` function with signature `addAdmin(address)` and selector `[112, 72, 2, 117]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addAdmin", abi = "addAdmin(address)")]
    pub struct AddAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `admins` function with signature `admins(address)` and selector `[66, 155, 98, 229]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "admins", abi = "admins(address)")]
    pub struct AdminsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `calculate` function with signature `calculate(uint256,address)` and selector `[90, 29, 36, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "calculate", abi = "calculate(uint256,address)")]
    pub struct CalculateCall {
        pub score: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `drainTo` function with signature `drainTo(address[],address)` and selector `[176, 144, 19, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "drainTo", abi = "drainTo(address[],address)")]
    pub struct DrainToCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub dest: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `max` function with signature `max()` and selector `[106, 197, 219, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "max", abi = "max()")]
    pub struct MaxCall;
    #[doc = "Container type for all input parameters for the `nonceUsed` function with signature `nonceUsed(address,uint256)` and selector `[22, 71, 121, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonceUsed", abi = "nonceUsed(address,uint256)")]
    pub struct NonceUsedCall {
        pub participant: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `removeAdmin` function with signature `removeAdmin(address)` and selector `[23, 133, 245, 60]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeAdmin", abi = "removeAdmin(address)")]
    pub struct RemoveAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `scale` function with signature `scale()` and selector `[245, 30, 24, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "scale", abi = "scale()")]
    pub struct ScaleCall;
    #[doc = "Container type for all input parameters for the `setMax` function with signature `setMax(uint256)` and selector `[31, 233, 234, 188]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMax", abi = "setMax(uint256)")]
    pub struct SetMaxCall {
        pub max: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setScale` function with signature `setScale(uint256)` and selector `[62, 220, 53, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setScale", abi = "setScale(uint256)")]
    pub struct SetScaleCall {
        pub scale: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStakingContract` function with signature `setStakingContract(address)` and selector `[157, 211, 115, 185]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStakingContract", abi = "setStakingContract(address)")]
    pub struct SetStakingContractCall {
        pub staking_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setStakingToken` function with signature `setStakingToken(address)` and selector `[30, 155, 18, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStakingToken", abi = "setStakingToken(address)")]
    pub struct SetStakingTokenCall {
        pub staking_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `stakingContract` function with signature `stakingContract()` and selector `[238, 153, 32, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakingContract", abi = "stakingContract()")]
    pub struct StakingContractCall;
    #[doc = "Container type for all input parameters for the `stakingToken` function with signature `stakingToken()` and selector `[114, 247, 2, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakingToken", abi = "stakingToken()")]
    pub struct StakingTokenCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `verify` function with signature `verify(uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[46, 15, 84, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "verify",
        abi = "verify(uint256,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct VerifyCall {
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub participant: ethers::core::types::Address,
        pub score: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[247, 200, 40, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "withdraw",
        abi = "withdraw(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct WithdrawCall {
        pub recipient: ethers::core::types::Address,
        pub minimum: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub score: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdrawAndStake` function with signature `withdrawAndStake(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[141, 243, 42, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "withdrawAndStake",
        abi = "withdrawAndStake(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct WithdrawAndStakeCall {
        pub recipient: ethers::core::types::Address,
        pub minimum: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub score: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolCalls {
        ClaimTypehash(ClaimTypehashCall),
        DomainChainId(DomainChainIdCall),
        DomainName(DomainNameCall),
        DomainSeparator(DomainSeparatorCall),
        DomainTypehash(DomainTypehashCall),
        DomainVersion(DomainVersionCall),
        AddAdmin(AddAdminCall),
        Admins(AdminsCall),
        Calculate(CalculateCall),
        DrainTo(DrainToCall),
        GetChainId(GetChainIdCall),
        Max(MaxCall),
        NonceUsed(NonceUsedCall),
        Owner(OwnerCall),
        RemoveAdmin(RemoveAdminCall),
        RenounceOwnership(RenounceOwnershipCall),
        Scale(ScaleCall),
        SetMax(SetMaxCall),
        SetScale(SetScaleCall),
        SetStakingContract(SetStakingContractCall),
        SetStakingToken(SetStakingTokenCall),
        StakingContract(StakingContractCall),
        StakingToken(StakingTokenCall),
        TransferOwnership(TransferOwnershipCall),
        Verify(VerifyCall),
        Withdraw(WithdrawCall),
        WithdrawAndStake(WithdrawAndStakeCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ClaimTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::ClaimTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DomainChainId(decoded));
            }
            if let Ok(decoded) =
                <DomainNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DomainName(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DomainVersion(decoded));
            }
            if let Ok(decoded) =
                <AddAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::AddAdmin(decoded));
            }
            if let Ok(decoded) = <AdminsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Admins(decoded));
            }
            if let Ok(decoded) =
                <CalculateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Calculate(decoded));
            }
            if let Ok(decoded) =
                <DrainToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DrainTo(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetChainId(decoded));
            }
            if let Ok(decoded) = <MaxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IPoolCalls::Max(decoded));
            }
            if let Ok(decoded) =
                <NonceUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::NonceUsed(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RemoveAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::RemoveAdmin(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Scale(decoded));
            }
            if let Ok(decoded) = <SetMaxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetMax(decoded));
            }
            if let Ok(decoded) =
                <SetScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetScale(decoded));
            }
            if let Ok(decoded) =
                <SetStakingContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetStakingContract(decoded));
            }
            if let Ok(decoded) =
                <SetStakingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetStakingToken(decoded));
            }
            if let Ok(decoded) =
                <StakingContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::StakingContract(decoded));
            }
            if let Ok(decoded) =
                <StakingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::StakingToken(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Verify(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::WithdrawAndStake(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolCalls::ClaimTypehash(element) => element.encode(),
                IPoolCalls::DomainChainId(element) => element.encode(),
                IPoolCalls::DomainName(element) => element.encode(),
                IPoolCalls::DomainSeparator(element) => element.encode(),
                IPoolCalls::DomainTypehash(element) => element.encode(),
                IPoolCalls::DomainVersion(element) => element.encode(),
                IPoolCalls::AddAdmin(element) => element.encode(),
                IPoolCalls::Admins(element) => element.encode(),
                IPoolCalls::Calculate(element) => element.encode(),
                IPoolCalls::DrainTo(element) => element.encode(),
                IPoolCalls::GetChainId(element) => element.encode(),
                IPoolCalls::Max(element) => element.encode(),
                IPoolCalls::NonceUsed(element) => element.encode(),
                IPoolCalls::Owner(element) => element.encode(),
                IPoolCalls::RemoveAdmin(element) => element.encode(),
                IPoolCalls::RenounceOwnership(element) => element.encode(),
                IPoolCalls::Scale(element) => element.encode(),
                IPoolCalls::SetMax(element) => element.encode(),
                IPoolCalls::SetScale(element) => element.encode(),
                IPoolCalls::SetStakingContract(element) => element.encode(),
                IPoolCalls::SetStakingToken(element) => element.encode(),
                IPoolCalls::StakingContract(element) => element.encode(),
                IPoolCalls::StakingToken(element) => element.encode(),
                IPoolCalls::TransferOwnership(element) => element.encode(),
                IPoolCalls::Verify(element) => element.encode(),
                IPoolCalls::Withdraw(element) => element.encode(),
                IPoolCalls::WithdrawAndStake(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolCalls::ClaimTypehash(element) => element.fmt(f),
                IPoolCalls::DomainChainId(element) => element.fmt(f),
                IPoolCalls::DomainName(element) => element.fmt(f),
                IPoolCalls::DomainSeparator(element) => element.fmt(f),
                IPoolCalls::DomainTypehash(element) => element.fmt(f),
                IPoolCalls::DomainVersion(element) => element.fmt(f),
                IPoolCalls::AddAdmin(element) => element.fmt(f),
                IPoolCalls::Admins(element) => element.fmt(f),
                IPoolCalls::Calculate(element) => element.fmt(f),
                IPoolCalls::DrainTo(element) => element.fmt(f),
                IPoolCalls::GetChainId(element) => element.fmt(f),
                IPoolCalls::Max(element) => element.fmt(f),
                IPoolCalls::NonceUsed(element) => element.fmt(f),
                IPoolCalls::Owner(element) => element.fmt(f),
                IPoolCalls::RemoveAdmin(element) => element.fmt(f),
                IPoolCalls::RenounceOwnership(element) => element.fmt(f),
                IPoolCalls::Scale(element) => element.fmt(f),
                IPoolCalls::SetMax(element) => element.fmt(f),
                IPoolCalls::SetScale(element) => element.fmt(f),
                IPoolCalls::SetStakingContract(element) => element.fmt(f),
                IPoolCalls::SetStakingToken(element) => element.fmt(f),
                IPoolCalls::StakingContract(element) => element.fmt(f),
                IPoolCalls::StakingToken(element) => element.fmt(f),
                IPoolCalls::TransferOwnership(element) => element.fmt(f),
                IPoolCalls::Verify(element) => element.fmt(f),
                IPoolCalls::Withdraw(element) => element.fmt(f),
                IPoolCalls::WithdrawAndStake(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ClaimTypehashCall> for IPoolCalls {
        fn from(var: ClaimTypehashCall) -> Self {
            IPoolCalls::ClaimTypehash(var)
        }
    }
    impl ::std::convert::From<DomainChainIdCall> for IPoolCalls {
        fn from(var: DomainChainIdCall) -> Self {
            IPoolCalls::DomainChainId(var)
        }
    }
    impl ::std::convert::From<DomainNameCall> for IPoolCalls {
        fn from(var: DomainNameCall) -> Self {
            IPoolCalls::DomainName(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for IPoolCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            IPoolCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<DomainTypehashCall> for IPoolCalls {
        fn from(var: DomainTypehashCall) -> Self {
            IPoolCalls::DomainTypehash(var)
        }
    }
    impl ::std::convert::From<DomainVersionCall> for IPoolCalls {
        fn from(var: DomainVersionCall) -> Self {
            IPoolCalls::DomainVersion(var)
        }
    }
    impl ::std::convert::From<AddAdminCall> for IPoolCalls {
        fn from(var: AddAdminCall) -> Self {
            IPoolCalls::AddAdmin(var)
        }
    }
    impl ::std::convert::From<AdminsCall> for IPoolCalls {
        fn from(var: AdminsCall) -> Self {
            IPoolCalls::Admins(var)
        }
    }
    impl ::std::convert::From<CalculateCall> for IPoolCalls {
        fn from(var: CalculateCall) -> Self {
            IPoolCalls::Calculate(var)
        }
    }
    impl ::std::convert::From<DrainToCall> for IPoolCalls {
        fn from(var: DrainToCall) -> Self {
            IPoolCalls::DrainTo(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for IPoolCalls {
        fn from(var: GetChainIdCall) -> Self {
            IPoolCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<MaxCall> for IPoolCalls {
        fn from(var: MaxCall) -> Self {
            IPoolCalls::Max(var)
        }
    }
    impl ::std::convert::From<NonceUsedCall> for IPoolCalls {
        fn from(var: NonceUsedCall) -> Self {
            IPoolCalls::NonceUsed(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IPoolCalls {
        fn from(var: OwnerCall) -> Self {
            IPoolCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RemoveAdminCall> for IPoolCalls {
        fn from(var: RemoveAdminCall) -> Self {
            IPoolCalls::RemoveAdmin(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for IPoolCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            IPoolCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<ScaleCall> for IPoolCalls {
        fn from(var: ScaleCall) -> Self {
            IPoolCalls::Scale(var)
        }
    }
    impl ::std::convert::From<SetMaxCall> for IPoolCalls {
        fn from(var: SetMaxCall) -> Self {
            IPoolCalls::SetMax(var)
        }
    }
    impl ::std::convert::From<SetScaleCall> for IPoolCalls {
        fn from(var: SetScaleCall) -> Self {
            IPoolCalls::SetScale(var)
        }
    }
    impl ::std::convert::From<SetStakingContractCall> for IPoolCalls {
        fn from(var: SetStakingContractCall) -> Self {
            IPoolCalls::SetStakingContract(var)
        }
    }
    impl ::std::convert::From<SetStakingTokenCall> for IPoolCalls {
        fn from(var: SetStakingTokenCall) -> Self {
            IPoolCalls::SetStakingToken(var)
        }
    }
    impl ::std::convert::From<StakingContractCall> for IPoolCalls {
        fn from(var: StakingContractCall) -> Self {
            IPoolCalls::StakingContract(var)
        }
    }
    impl ::std::convert::From<StakingTokenCall> for IPoolCalls {
        fn from(var: StakingTokenCall) -> Self {
            IPoolCalls::StakingToken(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for IPoolCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            IPoolCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<VerifyCall> for IPoolCalls {
        fn from(var: VerifyCall) -> Self {
            IPoolCalls::Verify(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IPoolCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAndStakeCall> for IPoolCalls {
        fn from(var: WithdrawAndStakeCall) -> Self {
            IPoolCalls::WithdrawAndStake(var)
        }
    }
    #[doc = "Container type for all return fields from the `CLAIM_TYPEHASH` function with signature `CLAIM_TYPEHASH()` and selector `[107, 5, 9, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ClaimTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_CHAIN_ID` function with signature `DOMAIN_CHAIN_ID()` and selector `[65, 111, 40, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainChainIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `DOMAIN_NAME` function with signature `DOMAIN_NAME()` and selector `[121, 111, 7, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainNameReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_VERSION` function with signature `DOMAIN_VERSION()` and selector `[172, 184, 204, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainVersionReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `admins` function with signature `admins(address)` and selector `[66, 155, 98, 229]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AdminsReturn(pub bool);
    #[doc = "Container type for all return fields from the `calculate` function with signature `calculate(uint256,address)` and selector `[90, 29, 36, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetChainIdReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `max` function with signature `max()` and selector `[106, 197, 219, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `nonceUsed` function with signature `nonceUsed(address,uint256)` and selector `[22, 71, 121, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NonceUsedReturn(pub bool);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `scale` function with signature `scale()` and selector `[245, 30, 24, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ScaleReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `stakingContract` function with signature `stakingContract()` and selector `[238, 153, 32, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StakingContractReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `stakingToken` function with signature `stakingToken()` and selector `[114, 247, 2, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StakingTokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `verify` function with signature `verify(uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[46, 15, 84, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyReturn {
        pub valid: bool,
    }
    #[doc = "Container type for all return fields from the `withdraw` function with signature `withdraw(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[247, 200, 40, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WithdrawReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `withdrawAndStake` function with signature `withdrawAndStake(address,uint256,address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[141, 243, 42, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WithdrawAndStakeReturn(pub ethers::core::types::U256);
}
