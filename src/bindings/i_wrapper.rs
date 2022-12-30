pub use i_wrapper::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_wrapper {
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
    #[doc = "IWrapper was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_swapContract\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_wethContract\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"senderWallet\",\"type\":\"address\"}],\"name\":\"WrappedSwapFor\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_swapContract\",\"type\":\"address\"}],\"name\":\"setSwapContract\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"swap\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"swapAnySender\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"swapContract\",\"outputs\":[{\"internalType\":\"contract ISwapERC20\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"wethContract\",\"outputs\":[{\"internalType\":\"contract IWETH\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IWRAPPER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IWrapper<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IWrapper<M> {
        fn clone(&self) -> Self {
            IWrapper(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IWrapper<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IWrapper<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IWrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IWrapper<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IWRAPPER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSwapContract` (0xd259ab42) function"]
        pub fn set_swap_contract(
            &self,
            swap_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 89, 171, 66], swap_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x8f8c57fe) function"]
        pub fn swap(
            &self,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            signer_wallet: ethers::core::types::Address,
            signer_token: ethers::core::types::Address,
            signer_amount: ethers::core::types::U256,
            sender_token: ethers::core::types::Address,
            sender_amount: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 140, 87, 254],
                    (
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
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAnySender` (0xc6f9b5fe) function"]
        pub fn swap_any_sender(
            &self,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            signer_wallet: ethers::core::types::Address,
            signer_token: ethers::core::types::Address,
            signer_amount: ethers::core::types::U256,
            sender_token: ethers::core::types::Address,
            sender_amount: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [198, 249, 181, 254],
                    (
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
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapContract` (0x8ea83031) function"]
        pub fn swap_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([142, 168, 48, 49], ())
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
        #[doc = "Calls the contract's `wethContract` (0x4780eac1) function"]
        pub fn weth_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([71, 128, 234, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `WrappedSwapFor` event"]
        pub fn wrapped_swap_for_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WrappedSwapForFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IWrapperEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IWrapper<M> {
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
    #[ethevent(name = "WrappedSwapFor", abi = "WrappedSwapFor(address)")]
    pub struct WrappedSwapForFilter {
        #[ethevent(indexed)]
        pub sender_wallet: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWrapperEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        WrappedSwapForFilter(WrappedSwapForFilter),
    }
    impl ethers::contract::EthLogDecode for IWrapperEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(IWrapperEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = WrappedSwapForFilter::decode_log(log) {
                return Ok(IWrapperEvents::WrappedSwapForFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IWrapperEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWrapperEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                IWrapperEvents::WrappedSwapForFilter(element) => element.fmt(f),
            }
        }
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
    #[doc = "Container type for all input parameters for the `setSwapContract` function with signature `setSwapContract(address)` and selector `[210, 89, 171, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setSwapContract", abi = "setSwapContract(address)")]
    pub struct SetSwapContractCall {
        pub swap_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[143, 140, 87, 254]`"]
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
        name = "swap",
        abi = "swap(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SwapCall {
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub signer_wallet: ethers::core::types::Address,
        pub signer_token: ethers::core::types::Address,
        pub signer_amount: ethers::core::types::U256,
        pub sender_token: ethers::core::types::Address,
        pub sender_amount: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapAnySender` function with signature `swapAnySender(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[198, 249, 181, 254]`"]
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
        name = "swapAnySender",
        abi = "swapAnySender(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SwapAnySenderCall {
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub signer_wallet: ethers::core::types::Address,
        pub signer_token: ethers::core::types::Address,
        pub signer_amount: ethers::core::types::U256,
        pub sender_token: ethers::core::types::Address,
        pub sender_amount: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapContract` function with signature `swapContract()` and selector `[142, 168, 48, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swapContract", abi = "swapContract()")]
    pub struct SwapContractCall;
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
    #[doc = "Container type for all input parameters for the `wethContract` function with signature `wethContract()` and selector `[71, 128, 234, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "wethContract", abi = "wethContract()")]
    pub struct WethContractCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWrapperCalls {
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetSwapContract(SetSwapContractCall),
        Swap(SwapCall),
        SwapAnySender(SwapAnySenderCall),
        SwapContract(SwapContractCall),
        TransferOwnership(TransferOwnershipCall),
        WethContract(WethContractCall),
    }
    impl ethers::core::abi::AbiDecode for IWrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetSwapContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::SetSwapContract(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IWrapperCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <SwapAnySenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::SwapAnySender(decoded));
            }
            if let Ok(decoded) =
                <SwapContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::SwapContract(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WethContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWrapperCalls::WethContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IWrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IWrapperCalls::Owner(element) => element.encode(),
                IWrapperCalls::RenounceOwnership(element) => element.encode(),
                IWrapperCalls::SetSwapContract(element) => element.encode(),
                IWrapperCalls::Swap(element) => element.encode(),
                IWrapperCalls::SwapAnySender(element) => element.encode(),
                IWrapperCalls::SwapContract(element) => element.encode(),
                IWrapperCalls::TransferOwnership(element) => element.encode(),
                IWrapperCalls::WethContract(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IWrapperCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWrapperCalls::Owner(element) => element.fmt(f),
                IWrapperCalls::RenounceOwnership(element) => element.fmt(f),
                IWrapperCalls::SetSwapContract(element) => element.fmt(f),
                IWrapperCalls::Swap(element) => element.fmt(f),
                IWrapperCalls::SwapAnySender(element) => element.fmt(f),
                IWrapperCalls::SwapContract(element) => element.fmt(f),
                IWrapperCalls::TransferOwnership(element) => element.fmt(f),
                IWrapperCalls::WethContract(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OwnerCall> for IWrapperCalls {
        fn from(var: OwnerCall) -> Self {
            IWrapperCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for IWrapperCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            IWrapperCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetSwapContractCall> for IWrapperCalls {
        fn from(var: SetSwapContractCall) -> Self {
            IWrapperCalls::SetSwapContract(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IWrapperCalls {
        fn from(var: SwapCall) -> Self {
            IWrapperCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SwapAnySenderCall> for IWrapperCalls {
        fn from(var: SwapAnySenderCall) -> Self {
            IWrapperCalls::SwapAnySender(var)
        }
    }
    impl ::std::convert::From<SwapContractCall> for IWrapperCalls {
        fn from(var: SwapContractCall) -> Self {
            IWrapperCalls::SwapContract(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for IWrapperCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            IWrapperCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WethContractCall> for IWrapperCalls {
        fn from(var: WethContractCall) -> Self {
            IWrapperCalls::WethContract(var)
        }
    }
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
    #[doc = "Container type for all return fields from the `swapContract` function with signature `swapContract()` and selector `[142, 168, 48, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapContractReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `wethContract` function with signature `wethContract()` and selector `[71, 128, 234, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WethContractReturn(pub ethers::core::types::Address);
}
