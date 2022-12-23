pub use staking::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod staking {
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
    #[doc = "Staking was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract ERC20\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"_duration\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_minDelay\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"CancelDurationChange\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"newDuration\",\"type\":\"uint256\"}],\"name\":\"CompleteDurationChange\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"ProposeDelegate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"unlockTimestamp\",\"type\":\"uint256\"}],\"name\":\"ScheduleDurationChange\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"SetDelegate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"tokens\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"accountDelegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"available\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"total\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"cancelDurationChange\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"delegateAccounts\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"duration\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"getStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"duration\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"internalType\":\"struct IStaking.Stake\",\"name\":\"accountStake\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\"}],\"name\":\"proposeDelegate\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"proposedDelegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delay\",\"type\":\"uint256\"}],\"name\":\"scheduleDurationChange\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"setDelegate\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_duration\",\"type\":\"uint256\"}],\"name\":\"setDuration\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\"}],\"name\":\"setMetaData\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"stake\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"stakeFor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"token\",\"outputs\":[{\"internalType\":\"contract ERC20\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\"}],\"name\":\"unsetDelegate\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"unstake\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static STAKING_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Staking<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Staking<M> {
        fn clone(&self) -> Self {
            Staking(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Staking<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Staking<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Staking))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Staking<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), STAKING_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `accountDelegates` (0x8f2318cb) function"]
        pub fn account_delegates(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([143, 35, 24, 203], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `available` (0x10098ad5) function"]
        pub fn available(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([16, 9, 138, 213], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelDurationChange` (0x9ef682d2) function"]
        pub fn cancel_duration_change(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 246, 130, 210], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateAccounts` (0x13838a02) function"]
        pub fn delegate_accounts(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([19, 131, 138, 2], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `duration` (0x0fb5a6b4) function"]
        pub fn duration(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 181, 166, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStakes` (0x7ba6f458) function"]
        pub fn get_stakes(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, Stake> {
            self.0
                .method_hash([123, 166, 244, 88], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `proposeDelegate` (0x96dcfbe1) function"]
        pub fn propose_delegate(
            &self,
            delegate: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 220, 251, 225], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedDelegates` (0x0b608fcb) function"]
        pub fn proposed_delegates(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([11, 96, 143, 203], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scheduleDurationChange` (0x20aaba3b) function"]
        pub fn schedule_duration_change(
            &self,
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 170, 186, 59], delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDelegate` (0xca5eb5e1) function"]
        pub fn set_delegate(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 94, 181, 225], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDuration` (0xf6be71d1) function"]
        pub fn set_duration(
            &self,
            duration: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 190, 113, 209], duration)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMetaData` (0xbb2c4100) function"]
        pub fn set_meta_data(
            &self,
            name: String,
            symbol: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 44, 65, 0], (name, symbol))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stake` (0xa694fc3a) function"]
        pub fn stake(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 148, 252, 58], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeFor` (0x2ee40908) function"]
        pub fn stake_for(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 228, 9, 8], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
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
        #[doc = "Calls the contract's `unsetDelegate` (0x826b971e) function"]
        pub fn unset_delegate(
            &self,
            delegate: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 107, 151, 30], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unstake` (0x2e17de78) function"]
        pub fn unstake(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 23, 222, 120], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CancelDurationChange` event"]
        pub fn cancel_duration_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelDurationChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CompleteDurationChange` event"]
        pub fn complete_duration_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CompleteDurationChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposeDelegate` event"]
        pub fn propose_delegate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposeDelegateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ScheduleDurationChange` event"]
        pub fn schedule_duration_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ScheduleDurationChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetDelegate` event"]
        pub fn set_delegate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetDelegateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, StakingEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Staking<M> {
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
    #[ethevent(name = "CancelDurationChange", abi = "CancelDurationChange()")]
    pub struct CancelDurationChangeFilter();
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
        name = "CompleteDurationChange",
        abi = "CompleteDurationChange(uint256)"
    )]
    pub struct CompleteDurationChangeFilter {
        #[ethevent(indexed)]
        pub new_duration: ethers::core::types::U256,
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
    #[ethevent(name = "ProposeDelegate", abi = "ProposeDelegate(address,address)")]
    pub struct ProposeDelegateFilter {
        #[ethevent(indexed)]
        pub delegate: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
        name = "ScheduleDurationChange",
        abi = "ScheduleDurationChange(uint256)"
    )]
    pub struct ScheduleDurationChangeFilter {
        #[ethevent(indexed)]
        pub unlock_timestamp: ethers::core::types::U256,
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
    #[ethevent(name = "SetDelegate", abi = "SetDelegate(address,address)")]
    pub struct SetDelegateFilter {
        #[ethevent(indexed)]
        pub delegate: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub tokens: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StakingEvents {
        CancelDurationChangeFilter(CancelDurationChangeFilter),
        CompleteDurationChangeFilter(CompleteDurationChangeFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProposeDelegateFilter(ProposeDelegateFilter),
        ScheduleDurationChangeFilter(ScheduleDurationChangeFilter),
        SetDelegateFilter(SetDelegateFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for StakingEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CancelDurationChangeFilter::decode_log(log) {
                return Ok(StakingEvents::CancelDurationChangeFilter(decoded));
            }
            if let Ok(decoded) = CompleteDurationChangeFilter::decode_log(log) {
                return Ok(StakingEvents::CompleteDurationChangeFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StakingEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProposeDelegateFilter::decode_log(log) {
                return Ok(StakingEvents::ProposeDelegateFilter(decoded));
            }
            if let Ok(decoded) = ScheduleDurationChangeFilter::decode_log(log) {
                return Ok(StakingEvents::ScheduleDurationChangeFilter(decoded));
            }
            if let Ok(decoded) = SetDelegateFilter::decode_log(log) {
                return Ok(StakingEvents::SetDelegateFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(StakingEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for StakingEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StakingEvents::CancelDurationChangeFilter(element) => element.fmt(f),
                StakingEvents::CompleteDurationChangeFilter(element) => element.fmt(f),
                StakingEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                StakingEvents::ProposeDelegateFilter(element) => element.fmt(f),
                StakingEvents::ScheduleDurationChangeFilter(element) => element.fmt(f),
                StakingEvents::SetDelegateFilter(element) => element.fmt(f),
                StakingEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `accountDelegates` function with signature `accountDelegates(address)` and selector `[143, 35, 24, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "accountDelegates", abi = "accountDelegates(address)")]
    pub struct AccountDelegatesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `available` function with signature `available(address)` and selector `[16, 9, 138, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "available", abi = "available(address)")]
    pub struct AvailableCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cancelDurationChange` function with signature `cancelDurationChange()` and selector `[158, 246, 130, 210]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelDurationChange", abi = "cancelDurationChange()")]
    pub struct CancelDurationChangeCall;
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `delegateAccounts` function with signature `delegateAccounts(address)` and selector `[19, 131, 138, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "delegateAccounts", abi = "delegateAccounts(address)")]
    pub struct DelegateAccountsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `duration` function with signature `duration()` and selector `[15, 181, 166, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "duration", abi = "duration()")]
    pub struct DurationCall;
    #[doc = "Container type for all input parameters for the `getStakes` function with signature `getStakes(address)` and selector `[123, 166, 244, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getStakes", abi = "getStakes(address)")]
    pub struct GetStakesCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    #[doc = "Container type for all input parameters for the `proposeDelegate` function with signature `proposeDelegate(address)` and selector `[150, 220, 251, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "proposeDelegate", abi = "proposeDelegate(address)")]
    pub struct ProposeDelegateCall {
        pub delegate: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `proposedDelegates` function with signature `proposedDelegates(address)` and selector `[11, 96, 143, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "proposedDelegates", abi = "proposedDelegates(address)")]
    pub struct ProposedDelegatesCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `scheduleDurationChange` function with signature `scheduleDurationChange(uint256)` and selector `[32, 170, 186, 59]`"]
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
        name = "scheduleDurationChange",
        abi = "scheduleDurationChange(uint256)"
    )]
    pub struct ScheduleDurationChangeCall {
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setDelegate` function with signature `setDelegate(address)` and selector `[202, 94, 181, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDelegate", abi = "setDelegate(address)")]
    pub struct SetDelegateCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDuration` function with signature `setDuration(uint256)` and selector `[246, 190, 113, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDuration", abi = "setDuration(uint256)")]
    pub struct SetDurationCall {
        pub duration: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMetaData` function with signature `setMetaData(string,string)` and selector `[187, 44, 65, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMetaData", abi = "setMetaData(string,string)")]
    pub struct SetMetaDataCall {
        pub name: String,
        pub symbol: String,
    }
    #[doc = "Container type for all input parameters for the `stake` function with signature `stake(uint256)` and selector `[166, 148, 252, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stake", abi = "stake(uint256)")]
    pub struct StakeCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `stakeFor` function with signature `stakeFor(address,uint256)` and selector `[46, 228, 9, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakeFor", abi = "stakeFor(address,uint256)")]
    pub struct StakeForCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
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
    #[doc = "Container type for all input parameters for the `unsetDelegate` function with signature `unsetDelegate(address)` and selector `[130, 107, 151, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unsetDelegate", abi = "unsetDelegate(address)")]
    pub struct UnsetDelegateCall {
        pub delegate: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unstake` function with signature `unstake(uint256)` and selector `[46, 23, 222, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unstake", abi = "unstake(uint256)")]
    pub struct UnstakeCall {
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StakingCalls {
        AccountDelegates(AccountDelegatesCall),
        Available(AvailableCall),
        BalanceOf(BalanceOfCall),
        CancelDurationChange(CancelDurationChangeCall),
        Decimals(DecimalsCall),
        DelegateAccounts(DelegateAccountsCall),
        Duration(DurationCall),
        GetStakes(GetStakesCall),
        Name(NameCall),
        Owner(OwnerCall),
        ProposeDelegate(ProposeDelegateCall),
        ProposedDelegates(ProposedDelegatesCall),
        RenounceOwnership(RenounceOwnershipCall),
        ScheduleDurationChange(ScheduleDurationChangeCall),
        SetDelegate(SetDelegateCall),
        SetDuration(SetDurationCall),
        SetMetaData(SetMetaDataCall),
        Stake(StakeCall),
        StakeFor(StakeForCall),
        Symbol(SymbolCall),
        Token(TokenCall),
        TotalSupply(TotalSupplyCall),
        TransferOwnership(TransferOwnershipCall),
        UnsetDelegate(UnsetDelegateCall),
        Unstake(UnstakeCall),
    }
    impl ethers::core::abi::AbiDecode for StakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccountDelegatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::AccountDelegates(decoded));
            }
            if let Ok(decoded) =
                <AvailableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Available(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CancelDurationChangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::CancelDurationChange(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DelegateAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::DelegateAccounts(decoded));
            }
            if let Ok(decoded) =
                <DurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Duration(decoded));
            }
            if let Ok(decoded) =
                <GetStakesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::GetStakes(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StakingCalls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ProposeDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::ProposeDelegate(decoded));
            }
            if let Ok(decoded) =
                <ProposedDelegatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::ProposedDelegates(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ScheduleDurationChangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::ScheduleDurationChange(decoded));
            }
            if let Ok(decoded) =
                <SetDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::SetDelegate(decoded));
            }
            if let Ok(decoded) =
                <SetDurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::SetDuration(decoded));
            }
            if let Ok(decoded) =
                <SetMetaDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::SetMetaData(decoded));
            }
            if let Ok(decoded) = <StakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Stake(decoded));
            }
            if let Ok(decoded) =
                <StakeForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::StakeFor(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnsetDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::UnsetDelegate(decoded));
            }
            if let Ok(decoded) =
                <UnstakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingCalls::Unstake(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for StakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                StakingCalls::AccountDelegates(element) => element.encode(),
                StakingCalls::Available(element) => element.encode(),
                StakingCalls::BalanceOf(element) => element.encode(),
                StakingCalls::CancelDurationChange(element) => element.encode(),
                StakingCalls::Decimals(element) => element.encode(),
                StakingCalls::DelegateAccounts(element) => element.encode(),
                StakingCalls::Duration(element) => element.encode(),
                StakingCalls::GetStakes(element) => element.encode(),
                StakingCalls::Name(element) => element.encode(),
                StakingCalls::Owner(element) => element.encode(),
                StakingCalls::ProposeDelegate(element) => element.encode(),
                StakingCalls::ProposedDelegates(element) => element.encode(),
                StakingCalls::RenounceOwnership(element) => element.encode(),
                StakingCalls::ScheduleDurationChange(element) => element.encode(),
                StakingCalls::SetDelegate(element) => element.encode(),
                StakingCalls::SetDuration(element) => element.encode(),
                StakingCalls::SetMetaData(element) => element.encode(),
                StakingCalls::Stake(element) => element.encode(),
                StakingCalls::StakeFor(element) => element.encode(),
                StakingCalls::Symbol(element) => element.encode(),
                StakingCalls::Token(element) => element.encode(),
                StakingCalls::TotalSupply(element) => element.encode(),
                StakingCalls::TransferOwnership(element) => element.encode(),
                StakingCalls::UnsetDelegate(element) => element.encode(),
                StakingCalls::Unstake(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for StakingCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StakingCalls::AccountDelegates(element) => element.fmt(f),
                StakingCalls::Available(element) => element.fmt(f),
                StakingCalls::BalanceOf(element) => element.fmt(f),
                StakingCalls::CancelDurationChange(element) => element.fmt(f),
                StakingCalls::Decimals(element) => element.fmt(f),
                StakingCalls::DelegateAccounts(element) => element.fmt(f),
                StakingCalls::Duration(element) => element.fmt(f),
                StakingCalls::GetStakes(element) => element.fmt(f),
                StakingCalls::Name(element) => element.fmt(f),
                StakingCalls::Owner(element) => element.fmt(f),
                StakingCalls::ProposeDelegate(element) => element.fmt(f),
                StakingCalls::ProposedDelegates(element) => element.fmt(f),
                StakingCalls::RenounceOwnership(element) => element.fmt(f),
                StakingCalls::ScheduleDurationChange(element) => element.fmt(f),
                StakingCalls::SetDelegate(element) => element.fmt(f),
                StakingCalls::SetDuration(element) => element.fmt(f),
                StakingCalls::SetMetaData(element) => element.fmt(f),
                StakingCalls::Stake(element) => element.fmt(f),
                StakingCalls::StakeFor(element) => element.fmt(f),
                StakingCalls::Symbol(element) => element.fmt(f),
                StakingCalls::Token(element) => element.fmt(f),
                StakingCalls::TotalSupply(element) => element.fmt(f),
                StakingCalls::TransferOwnership(element) => element.fmt(f),
                StakingCalls::UnsetDelegate(element) => element.fmt(f),
                StakingCalls::Unstake(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccountDelegatesCall> for StakingCalls {
        fn from(var: AccountDelegatesCall) -> Self {
            StakingCalls::AccountDelegates(var)
        }
    }
    impl ::std::convert::From<AvailableCall> for StakingCalls {
        fn from(var: AvailableCall) -> Self {
            StakingCalls::Available(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for StakingCalls {
        fn from(var: BalanceOfCall) -> Self {
            StakingCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CancelDurationChangeCall> for StakingCalls {
        fn from(var: CancelDurationChangeCall) -> Self {
            StakingCalls::CancelDurationChange(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for StakingCalls {
        fn from(var: DecimalsCall) -> Self {
            StakingCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DelegateAccountsCall> for StakingCalls {
        fn from(var: DelegateAccountsCall) -> Self {
            StakingCalls::DelegateAccounts(var)
        }
    }
    impl ::std::convert::From<DurationCall> for StakingCalls {
        fn from(var: DurationCall) -> Self {
            StakingCalls::Duration(var)
        }
    }
    impl ::std::convert::From<GetStakesCall> for StakingCalls {
        fn from(var: GetStakesCall) -> Self {
            StakingCalls::GetStakes(var)
        }
    }
    impl ::std::convert::From<NameCall> for StakingCalls {
        fn from(var: NameCall) -> Self {
            StakingCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for StakingCalls {
        fn from(var: OwnerCall) -> Self {
            StakingCalls::Owner(var)
        }
    }
    impl ::std::convert::From<ProposeDelegateCall> for StakingCalls {
        fn from(var: ProposeDelegateCall) -> Self {
            StakingCalls::ProposeDelegate(var)
        }
    }
    impl ::std::convert::From<ProposedDelegatesCall> for StakingCalls {
        fn from(var: ProposedDelegatesCall) -> Self {
            StakingCalls::ProposedDelegates(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for StakingCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            StakingCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<ScheduleDurationChangeCall> for StakingCalls {
        fn from(var: ScheduleDurationChangeCall) -> Self {
            StakingCalls::ScheduleDurationChange(var)
        }
    }
    impl ::std::convert::From<SetDelegateCall> for StakingCalls {
        fn from(var: SetDelegateCall) -> Self {
            StakingCalls::SetDelegate(var)
        }
    }
    impl ::std::convert::From<SetDurationCall> for StakingCalls {
        fn from(var: SetDurationCall) -> Self {
            StakingCalls::SetDuration(var)
        }
    }
    impl ::std::convert::From<SetMetaDataCall> for StakingCalls {
        fn from(var: SetMetaDataCall) -> Self {
            StakingCalls::SetMetaData(var)
        }
    }
    impl ::std::convert::From<StakeCall> for StakingCalls {
        fn from(var: StakeCall) -> Self {
            StakingCalls::Stake(var)
        }
    }
    impl ::std::convert::From<StakeForCall> for StakingCalls {
        fn from(var: StakeForCall) -> Self {
            StakingCalls::StakeFor(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for StakingCalls {
        fn from(var: SymbolCall) -> Self {
            StakingCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenCall> for StakingCalls {
        fn from(var: TokenCall) -> Self {
            StakingCalls::Token(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for StakingCalls {
        fn from(var: TotalSupplyCall) -> Self {
            StakingCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for StakingCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            StakingCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnsetDelegateCall> for StakingCalls {
        fn from(var: UnsetDelegateCall) -> Self {
            StakingCalls::UnsetDelegate(var)
        }
    }
    impl ::std::convert::From<UnstakeCall> for StakingCalls {
        fn from(var: UnstakeCall) -> Self {
            StakingCalls::Unstake(var)
        }
    }
    #[doc = "Container type for all return fields from the `accountDelegates` function with signature `accountDelegates(address)` and selector `[143, 35, 24, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AccountDelegatesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `available` function with signature `available(address)` and selector `[16, 9, 138, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AvailableReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn {
        pub total: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `delegateAccounts` function with signature `delegateAccounts(address)` and selector `[19, 131, 138, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DelegateAccountsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `duration` function with signature `duration()` and selector `[15, 181, 166, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DurationReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getStakes` function with signature `getStakes(address)` and selector `[123, 166, 244, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetStakesReturn {
        pub account_stake: Stake,
    }
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
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
    #[doc = "Container type for all return fields from the `proposedDelegates` function with signature `proposedDelegates(address)` and selector `[11, 96, 143, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProposedDelegatesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "`Stake(uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Stake {
        pub duration: ethers::core::types::U256,
        pub balance: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
    }
}
