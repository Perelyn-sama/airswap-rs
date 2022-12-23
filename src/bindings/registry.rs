pub use registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod registry {
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
    #[doc = "Registry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"_stakingToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_obligationCost\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_tokenCost\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"}],\"name\":\"AddTokens\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"FullUnstake\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"InitialStake\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"}],\"name\":\"RemoveTokens\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"url\",\"type\":\"string\"}],\"name\":\"SetURL\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"}],\"name\":\"addTokens\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"staker\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"getStakersForToken\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"stakers\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"staker\",\"type\":\"address\"}],\"name\":\"getSupportedTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"tokenList\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"stakers\",\"type\":\"address[]\"}],\"name\":\"getURLsForStakers\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"urls\",\"type\":\"string[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"getURLsForToken\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"urls\",\"type\":\"string[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"obligationCost\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"removeAllTokens\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\"}],\"name\":\"removeTokens\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_url\",\"type\":\"string\"}],\"name\":\"setURL\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"stakerURLs\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"stakingToken\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"staker\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"supportsToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"tokenCost\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static REGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Registry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Registry<M> {
        fn clone(&self) -> Self {
            Registry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Registry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Registry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Registry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Registry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), REGISTRY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addTokens` (0x4ae05c7d) function"]
        pub fn add_tokens(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 224, 92, 125], tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            staker: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], staker)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStakersForToken` (0x6e8658a7) function"]
        pub fn get_stakers_for_token(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([110, 134, 88, 167], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupportedTokens` (0x07526acf) function"]
        pub fn get_supported_tokens(
            &self,
            staker: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([7, 82, 106, 207], staker)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getURLsForStakers` (0xd12877dc) function"]
        pub fn get_ur_ls_for_stakers(
            &self,
            stakers: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([209, 40, 119, 220], stakers)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getURLsForToken` (0x6df35874) function"]
        pub fn get_ur_ls_for_token(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([109, 243, 88, 116], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `obligationCost` (0x2b24ef55) function"]
        pub fn obligation_cost(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([43, 36, 239, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeAllTokens` (0x793cc5a1) function"]
        pub fn remove_all_tokens(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 60, 197, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeTokens` (0x6c3824ef) function"]
        pub fn remove_tokens(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 56, 36, 239], tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setURL` (0x77343408) function"]
        pub fn set_url(&self, url: String) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 52, 52, 8], url)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakerURLs` (0xc3ced152) function"]
        pub fn staker_ur_ls(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([195, 206, 209, 82], p0)
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
        #[doc = "Calls the contract's `supportsToken` (0x53731c69) function"]
        pub fn supports_token(
            &self,
            staker: ethers::core::types::Address,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([83, 115, 28, 105], (staker, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenCost` (0x912221d5) function"]
        pub fn token_cost(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 34, 33, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddTokens` event"]
        pub fn add_tokens_filter(&self) -> ethers::contract::builders::Event<M, AddTokensFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FullUnstake` event"]
        pub fn full_unstake_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FullUnstakeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InitialStake` event"]
        pub fn initial_stake_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitialStakeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemoveTokens` event"]
        pub fn remove_tokens_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemoveTokensFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetURL` event"]
        pub fn set_url_filter(&self) -> ethers::contract::builders::Event<M, SetURLFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Registry<M> {
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
    #[ethevent(name = "AddTokens", abi = "AddTokens(address,address[])")]
    pub struct AddTokensFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub tokens: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "FullUnstake", abi = "FullUnstake(address)")]
    pub struct FullUnstakeFilter {
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
    #[ethevent(name = "InitialStake", abi = "InitialStake(address)")]
    pub struct InitialStakeFilter {
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
    #[ethevent(name = "RemoveTokens", abi = "RemoveTokens(address,address[])")]
    pub struct RemoveTokensFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub tokens: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "SetURL", abi = "SetURL(address,string)")]
    pub struct SetURLFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub url: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RegistryEvents {
        AddTokensFilter(AddTokensFilter),
        FullUnstakeFilter(FullUnstakeFilter),
        InitialStakeFilter(InitialStakeFilter),
        RemoveTokensFilter(RemoveTokensFilter),
        SetURLFilter(SetURLFilter),
    }
    impl ethers::contract::EthLogDecode for RegistryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddTokensFilter::decode_log(log) {
                return Ok(RegistryEvents::AddTokensFilter(decoded));
            }
            if let Ok(decoded) = FullUnstakeFilter::decode_log(log) {
                return Ok(RegistryEvents::FullUnstakeFilter(decoded));
            }
            if let Ok(decoded) = InitialStakeFilter::decode_log(log) {
                return Ok(RegistryEvents::InitialStakeFilter(decoded));
            }
            if let Ok(decoded) = RemoveTokensFilter::decode_log(log) {
                return Ok(RegistryEvents::RemoveTokensFilter(decoded));
            }
            if let Ok(decoded) = SetURLFilter::decode_log(log) {
                return Ok(RegistryEvents::SetURLFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for RegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RegistryEvents::AddTokensFilter(element) => element.fmt(f),
                RegistryEvents::FullUnstakeFilter(element) => element.fmt(f),
                RegistryEvents::InitialStakeFilter(element) => element.fmt(f),
                RegistryEvents::RemoveTokensFilter(element) => element.fmt(f),
                RegistryEvents::SetURLFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addTokens` function with signature `addTokens(address[])` and selector `[74, 224, 92, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addTokens", abi = "addTokens(address[])")]
    pub struct AddTokensCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
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
        pub staker: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getStakersForToken` function with signature `getStakersForToken(address)` and selector `[110, 134, 88, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getStakersForToken", abi = "getStakersForToken(address)")]
    pub struct GetStakersForTokenCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSupportedTokens` function with signature `getSupportedTokens(address)` and selector `[7, 82, 106, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSupportedTokens", abi = "getSupportedTokens(address)")]
    pub struct GetSupportedTokensCall {
        pub staker: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getURLsForStakers` function with signature `getURLsForStakers(address[])` and selector `[209, 40, 119, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getURLsForStakers", abi = "getURLsForStakers(address[])")]
    pub struct GetURLsForStakersCall {
        pub stakers: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getURLsForToken` function with signature `getURLsForToken(address)` and selector `[109, 243, 88, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getURLsForToken", abi = "getURLsForToken(address)")]
    pub struct GetURLsForTokenCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `obligationCost` function with signature `obligationCost()` and selector `[43, 36, 239, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "obligationCost", abi = "obligationCost()")]
    pub struct ObligationCostCall;
    #[doc = "Container type for all input parameters for the `removeAllTokens` function with signature `removeAllTokens()` and selector `[121, 60, 197, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeAllTokens", abi = "removeAllTokens()")]
    pub struct RemoveAllTokensCall;
    #[doc = "Container type for all input parameters for the `removeTokens` function with signature `removeTokens(address[])` and selector `[108, 56, 36, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeTokens", abi = "removeTokens(address[])")]
    pub struct RemoveTokensCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setURL` function with signature `setURL(string)` and selector `[119, 52, 52, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setURL", abi = "setURL(string)")]
    pub struct SetURLCall {
        pub url: String,
    }
    #[doc = "Container type for all input parameters for the `stakerURLs` function with signature `stakerURLs(address)` and selector `[195, 206, 209, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakerURLs", abi = "stakerURLs(address)")]
    pub struct StakerURLsCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `supportsToken` function with signature `supportsToken(address,address)` and selector `[83, 115, 28, 105]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsToken", abi = "supportsToken(address,address)")]
    pub struct SupportsTokenCall {
        pub staker: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `tokenCost` function with signature `tokenCost()` and selector `[145, 34, 33, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenCost", abi = "tokenCost()")]
    pub struct TokenCostCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RegistryCalls {
        AddTokens(AddTokensCall),
        BalanceOf(BalanceOfCall),
        GetStakersForToken(GetStakersForTokenCall),
        GetSupportedTokens(GetSupportedTokensCall),
        GetURLsForStakers(GetURLsForStakersCall),
        GetURLsForToken(GetURLsForTokenCall),
        ObligationCost(ObligationCostCall),
        RemoveAllTokens(RemoveAllTokensCall),
        RemoveTokens(RemoveTokensCall),
        SetURL(SetURLCall),
        StakerURLs(StakerURLsCall),
        StakingToken(StakingTokenCall),
        SupportsToken(SupportsTokenCall),
        TokenCost(TokenCostCall),
    }
    impl ethers::core::abi::AbiDecode for RegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::AddTokens(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <GetStakersForTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::GetStakersForToken(decoded));
            }
            if let Ok(decoded) =
                <GetSupportedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::GetSupportedTokens(decoded));
            }
            if let Ok(decoded) =
                <GetURLsForStakersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::GetURLsForStakers(decoded));
            }
            if let Ok(decoded) =
                <GetURLsForTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::GetURLsForToken(decoded));
            }
            if let Ok(decoded) =
                <ObligationCostCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::ObligationCost(decoded));
            }
            if let Ok(decoded) =
                <RemoveAllTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::RemoveAllTokens(decoded));
            }
            if let Ok(decoded) =
                <RemoveTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::RemoveTokens(decoded));
            }
            if let Ok(decoded) = <SetURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::SetURL(decoded));
            }
            if let Ok(decoded) =
                <StakerURLsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::StakerURLs(decoded));
            }
            if let Ok(decoded) =
                <StakingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::StakingToken(decoded));
            }
            if let Ok(decoded) =
                <SupportsTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::SupportsToken(decoded));
            }
            if let Ok(decoded) =
                <TokenCostCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RegistryCalls::TokenCost(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for RegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                RegistryCalls::AddTokens(element) => element.encode(),
                RegistryCalls::BalanceOf(element) => element.encode(),
                RegistryCalls::GetStakersForToken(element) => element.encode(),
                RegistryCalls::GetSupportedTokens(element) => element.encode(),
                RegistryCalls::GetURLsForStakers(element) => element.encode(),
                RegistryCalls::GetURLsForToken(element) => element.encode(),
                RegistryCalls::ObligationCost(element) => element.encode(),
                RegistryCalls::RemoveAllTokens(element) => element.encode(),
                RegistryCalls::RemoveTokens(element) => element.encode(),
                RegistryCalls::SetURL(element) => element.encode(),
                RegistryCalls::StakerURLs(element) => element.encode(),
                RegistryCalls::StakingToken(element) => element.encode(),
                RegistryCalls::SupportsToken(element) => element.encode(),
                RegistryCalls::TokenCost(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for RegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RegistryCalls::AddTokens(element) => element.fmt(f),
                RegistryCalls::BalanceOf(element) => element.fmt(f),
                RegistryCalls::GetStakersForToken(element) => element.fmt(f),
                RegistryCalls::GetSupportedTokens(element) => element.fmt(f),
                RegistryCalls::GetURLsForStakers(element) => element.fmt(f),
                RegistryCalls::GetURLsForToken(element) => element.fmt(f),
                RegistryCalls::ObligationCost(element) => element.fmt(f),
                RegistryCalls::RemoveAllTokens(element) => element.fmt(f),
                RegistryCalls::RemoveTokens(element) => element.fmt(f),
                RegistryCalls::SetURL(element) => element.fmt(f),
                RegistryCalls::StakerURLs(element) => element.fmt(f),
                RegistryCalls::StakingToken(element) => element.fmt(f),
                RegistryCalls::SupportsToken(element) => element.fmt(f),
                RegistryCalls::TokenCost(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddTokensCall> for RegistryCalls {
        fn from(var: AddTokensCall) -> Self {
            RegistryCalls::AddTokens(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for RegistryCalls {
        fn from(var: BalanceOfCall) -> Self {
            RegistryCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<GetStakersForTokenCall> for RegistryCalls {
        fn from(var: GetStakersForTokenCall) -> Self {
            RegistryCalls::GetStakersForToken(var)
        }
    }
    impl ::std::convert::From<GetSupportedTokensCall> for RegistryCalls {
        fn from(var: GetSupportedTokensCall) -> Self {
            RegistryCalls::GetSupportedTokens(var)
        }
    }
    impl ::std::convert::From<GetURLsForStakersCall> for RegistryCalls {
        fn from(var: GetURLsForStakersCall) -> Self {
            RegistryCalls::GetURLsForStakers(var)
        }
    }
    impl ::std::convert::From<GetURLsForTokenCall> for RegistryCalls {
        fn from(var: GetURLsForTokenCall) -> Self {
            RegistryCalls::GetURLsForToken(var)
        }
    }
    impl ::std::convert::From<ObligationCostCall> for RegistryCalls {
        fn from(var: ObligationCostCall) -> Self {
            RegistryCalls::ObligationCost(var)
        }
    }
    impl ::std::convert::From<RemoveAllTokensCall> for RegistryCalls {
        fn from(var: RemoveAllTokensCall) -> Self {
            RegistryCalls::RemoveAllTokens(var)
        }
    }
    impl ::std::convert::From<RemoveTokensCall> for RegistryCalls {
        fn from(var: RemoveTokensCall) -> Self {
            RegistryCalls::RemoveTokens(var)
        }
    }
    impl ::std::convert::From<SetURLCall> for RegistryCalls {
        fn from(var: SetURLCall) -> Self {
            RegistryCalls::SetURL(var)
        }
    }
    impl ::std::convert::From<StakerURLsCall> for RegistryCalls {
        fn from(var: StakerURLsCall) -> Self {
            RegistryCalls::StakerURLs(var)
        }
    }
    impl ::std::convert::From<StakingTokenCall> for RegistryCalls {
        fn from(var: StakingTokenCall) -> Self {
            RegistryCalls::StakingToken(var)
        }
    }
    impl ::std::convert::From<SupportsTokenCall> for RegistryCalls {
        fn from(var: SupportsTokenCall) -> Self {
            RegistryCalls::SupportsToken(var)
        }
    }
    impl ::std::convert::From<TokenCostCall> for RegistryCalls {
        fn from(var: TokenCostCall) -> Self {
            RegistryCalls::TokenCost(var)
        }
    }
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
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getStakersForToken` function with signature `getStakersForToken(address)` and selector `[110, 134, 88, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetStakersForTokenReturn {
        pub stakers: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `getSupportedTokens` function with signature `getSupportedTokens(address)` and selector `[7, 82, 106, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSupportedTokensReturn {
        pub token_list: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `getURLsForStakers` function with signature `getURLsForStakers(address[])` and selector `[209, 40, 119, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetURLsForStakersReturn {
        pub urls: ::std::vec::Vec<String>,
    }
    #[doc = "Container type for all return fields from the `getURLsForToken` function with signature `getURLsForToken(address)` and selector `[109, 243, 88, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetURLsForTokenReturn {
        pub urls: ::std::vec::Vec<String>,
    }
    #[doc = "Container type for all return fields from the `obligationCost` function with signature `obligationCost()` and selector `[43, 36, 239, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ObligationCostReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `stakerURLs` function with signature `stakerURLs(address)` and selector `[195, 206, 209, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StakerURLsReturn(pub String);
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
    #[doc = "Container type for all return fields from the `supportsToken` function with signature `supportsToken(address,address)` and selector `[83, 115, 28, 105]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsTokenReturn(pub bool);
    #[doc = "Container type for all return fields from the `tokenCost` function with signature `tokenCost()` and selector `[145, 34, 33, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenCostReturn(pub ethers::core::types::U256);
}
