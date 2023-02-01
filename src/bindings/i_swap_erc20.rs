pub use i_swap_erc20::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_swap_erc20 {
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
    #[doc = "ISwapERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_protocolFee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_protocolFeeLight\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_protocolFeeWallet\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_rebateScale\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_rebateMax\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_staking\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"}],\"name\":\"Authorize\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"}],\"name\":\"Cancel\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"}],\"name\":\"Revoke\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\"}],\"name\":\"SetProtocolFee\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"protocolFeeLight\",\"type\":\"uint256\"}],\"name\":\"SetProtocolFeeLight\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"feeWallet\",\"type\":\"address\"}],\"name\":\"SetProtocolFeeWallet\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"rebateMax\",\"type\":\"uint256\"}],\"name\":\"SetRebateMax\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"rebateScale\",\"type\":\"uint256\"}],\"name\":\"SetRebateScale\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"staking\",\"type\":\"address\"}],\"name\":\"SetStaking\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"senderWallet\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"}],\"name\":\"Swap\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DOMAIN_CHAIN_ID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_NAME\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"DOMAIN_VERSION\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"FEE_DIVISOR\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ORDER_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"signer\",\"type\":\"address\"}],\"name\":\"authorize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"authorized\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"stakingBalance\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\"}],\"name\":\"calculateDiscount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"wallet\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"calculateProtocolFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"nonces\",\"type\":\"uint256[]\"}],\"name\":\"cancel\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"senderWallet\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"check\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"signer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"name\":\"nonceUsed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"protocolFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"protocolFeeLight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"protocolFeeWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"rebateMax\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"rebateScale\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"revoke\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_protocolFee\",\"type\":\"uint256\"}],\"name\":\"setProtocolFee\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_protocolFeeLight\",\"type\":\"uint256\"}],\"name\":\"setProtocolFeeLight\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_protocolFeeWallet\",\"type\":\"address\"}],\"name\":\"setProtocolFeeWallet\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_rebateMax\",\"type\":\"uint256\"}],\"name\":\"setRebateMax\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_rebateScale\",\"type\":\"uint256\"}],\"name\":\"setRebateScale\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newstaking\",\"type\":\"address\"}],\"name\":\"setStaking\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"staking\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"swap\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"swapAnySender\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"signerWallet\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"signerAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"senderToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"senderAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"swapLight\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ISWAPERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ISwapERC20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ISwapERC20<M> {
        fn clone(&self) -> Self {
            ISwapERC20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ISwapERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ISwapERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ISwapERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ISwapERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISWAPERC20_ABI.clone(), client).into()
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
        #[doc = "Calls the contract's `FEE_DIVISOR` (0x9e93ad8e) function"]
        pub fn fee_divisor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([158, 147, 173, 142], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ORDER_TYPEHASH` (0xf973a209) function"]
        pub fn order_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([249, 115, 162, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorize` (0xb6a5d7de) function"]
        pub fn authorize(
            &self,
            signer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 165, 215, 222], signer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorized` (0xb9181611) function"]
        pub fn authorized(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([185, 24, 22, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateDiscount` (0x4d2af2b2) function"]
        pub fn calculate_discount(
            &self,
            staking_balance: ethers::core::types::U256,
            fee_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 42, 242, 178], (staking_balance, fee_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateProtocolFee` (0x52c5f1f5) function"]
        pub fn calculate_protocol_fee(
            &self,
            wallet: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([82, 197, 241, 245], (wallet, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancel` (0x2e340823) function"]
        pub fn cancel(
            &self,
            nonces: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 52, 8, 35], nonces)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `check` (0xb9cb01b0) function"]
        pub fn check(
            &self,
            sender_wallet: ethers::core::types::Address,
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
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash(
                    [185, 203, 1, 176],
                    (
                        sender_wallet,
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
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonceUsed` (0x1647795e) function"]
        pub fn nonce_used(
            &self,
            signer: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([22, 71, 121, 94], (signer, nonce))
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
        #[doc = "Calls the contract's `protocolFee` (0xb0e21e8a) function"]
        pub fn protocol_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([176, 226, 30, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolFeeLight` (0xf4ebc699) function"]
        pub fn protocol_fee_light(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 235, 198, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolFeeWallet` (0xcbf7c6c3) function"]
        pub fn protocol_fee_wallet(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([203, 247, 198, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebateMax` (0x770fde12) function"]
        pub fn rebate_max(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([119, 15, 222, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebateScale` (0xdb985cd9) function"]
        pub fn rebate_scale(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 152, 92, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revoke` (0xb6549f75) function"]
        pub fn revoke(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 84, 159, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setProtocolFee` (0x787dce3d) function"]
        pub fn set_protocol_fee(
            &self,
            protocol_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 125, 206, 61], protocol_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setProtocolFeeLight` (0xbfd4e557) function"]
        pub fn set_protocol_fee_light(
            &self,
            protocol_fee_light: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 212, 229, 87], protocol_fee_light)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setProtocolFeeWallet` (0x7ce78525) function"]
        pub fn set_protocol_fee_wallet(
            &self,
            protocol_fee_wallet: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 231, 133, 37], protocol_fee_wallet)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRebateMax` (0xd31eaa83) function"]
        pub fn set_rebate_max(
            &self,
            rebate_max: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 30, 170, 131], rebate_max)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRebateScale` (0xe95b771c) function"]
        pub fn set_rebate_scale(
            &self,
            rebate_scale: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 91, 119, 28], rebate_scale)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStaking` (0x8ff39099) function"]
        pub fn set_staking(
            &self,
            newstaking: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 243, 144, 153], newstaking)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `staking` (0x4cf088d9) function"]
        pub fn staking(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([76, 240, 136, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x98956069) function"]
        pub fn swap(
            &self,
            recipient: ethers::core::types::Address,
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
                    [152, 149, 96, 105],
                    (
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
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAnySender` (0x3eb1af24) function"]
        pub fn swap_any_sender(
            &self,
            recipient: ethers::core::types::Address,
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
                    [62, 177, 175, 36],
                    (
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
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapLight` (0x46e4480d) function"]
        pub fn swap_light(
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
                    [70, 228, 72, 13],
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Authorize` event"]
        pub fn authorize_filter(&self) -> ethers::contract::builders::Event<M, AuthorizeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Cancel` event"]
        pub fn cancel_filter(&self) -> ethers::contract::builders::Event<M, CancelFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Revoke` event"]
        pub fn revoke_filter(&self) -> ethers::contract::builders::Event<M, RevokeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetProtocolFee` event"]
        pub fn set_protocol_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetProtocolFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetProtocolFeeLight` event"]
        pub fn set_protocol_fee_light_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetProtocolFeeLightFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetProtocolFeeWallet` event"]
        pub fn set_protocol_fee_wallet_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetProtocolFeeWalletFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetRebateMax` event"]
        pub fn set_rebate_max_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetRebateMaxFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetRebateScale` event"]
        pub fn set_rebate_scale_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetRebateScaleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetStaking` event"]
        pub fn set_staking_filter(&self) -> ethers::contract::builders::Event<M, SetStakingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ISwapERC20Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ISwapERC20<M> {
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
    #[ethevent(name = "Authorize", abi = "Authorize(address,address)")]
    pub struct AuthorizeFilter {
        #[ethevent(indexed)]
        pub signer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub signer_wallet: ethers::core::types::Address,
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
    #[ethevent(name = "Cancel", abi = "Cancel(uint256,address)")]
    pub struct CancelFilter {
        #[ethevent(indexed)]
        pub nonce: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub signer_wallet: ethers::core::types::Address,
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
    #[ethevent(name = "Revoke", abi = "Revoke(address,address)")]
    pub struct RevokeFilter {
        #[ethevent(indexed)]
        pub signer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub signer_wallet: ethers::core::types::Address,
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
    #[ethevent(name = "SetProtocolFee", abi = "SetProtocolFee(uint256)")]
    pub struct SetProtocolFeeFilter {
        pub protocol_fee: ethers::core::types::U256,
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
    #[ethevent(name = "SetProtocolFeeLight", abi = "SetProtocolFeeLight(uint256)")]
    pub struct SetProtocolFeeLightFilter {
        pub protocol_fee_light: ethers::core::types::U256,
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
    #[ethevent(name = "SetProtocolFeeWallet", abi = "SetProtocolFeeWallet(address)")]
    pub struct SetProtocolFeeWalletFilter {
        #[ethevent(indexed)]
        pub fee_wallet: ethers::core::types::Address,
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
    #[ethevent(name = "SetRebateMax", abi = "SetRebateMax(uint256)")]
    pub struct SetRebateMaxFilter {
        pub rebate_max: ethers::core::types::U256,
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
    #[ethevent(name = "SetRebateScale", abi = "SetRebateScale(uint256)")]
    pub struct SetRebateScaleFilter {
        pub rebate_scale: ethers::core::types::U256,
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
    #[ethevent(name = "SetStaking", abi = "SetStaking(address)")]
    pub struct SetStakingFilter {
        #[ethevent(indexed)]
        pub staking: ethers::core::types::Address,
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
        name = "Swap",
        abi = "Swap(uint256,uint256,address,address,uint256,uint256,address,address,uint256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub nonce: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub signer_wallet: ethers::core::types::Address,
        pub signer_token: ethers::core::types::Address,
        pub signer_amount: ethers::core::types::U256,
        pub protocol_fee: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender_wallet: ethers::core::types::Address,
        pub sender_token: ethers::core::types::Address,
        pub sender_amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ISwapERC20Events {
        AuthorizeFilter(AuthorizeFilter),
        CancelFilter(CancelFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RevokeFilter(RevokeFilter),
        SetProtocolFeeFilter(SetProtocolFeeFilter),
        SetProtocolFeeLightFilter(SetProtocolFeeLightFilter),
        SetProtocolFeeWalletFilter(SetProtocolFeeWalletFilter),
        SetRebateMaxFilter(SetRebateMaxFilter),
        SetRebateScaleFilter(SetRebateScaleFilter),
        SetStakingFilter(SetStakingFilter),
        SwapFilter(SwapFilter),
    }
    impl ethers::contract::EthLogDecode for ISwapERC20Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuthorizeFilter::decode_log(log) {
                return Ok(ISwapERC20Events::AuthorizeFilter(decoded));
            }
            if let Ok(decoded) = CancelFilter::decode_log(log) {
                return Ok(ISwapERC20Events::CancelFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ISwapERC20Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RevokeFilter::decode_log(log) {
                return Ok(ISwapERC20Events::RevokeFilter(decoded));
            }
            if let Ok(decoded) = SetProtocolFeeFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetProtocolFeeFilter(decoded));
            }
            if let Ok(decoded) = SetProtocolFeeLightFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetProtocolFeeLightFilter(decoded));
            }
            if let Ok(decoded) = SetProtocolFeeWalletFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetProtocolFeeWalletFilter(decoded));
            }
            if let Ok(decoded) = SetRebateMaxFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetRebateMaxFilter(decoded));
            }
            if let Ok(decoded) = SetRebateScaleFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetRebateScaleFilter(decoded));
            }
            if let Ok(decoded) = SetStakingFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SetStakingFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(ISwapERC20Events::SwapFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ISwapERC20Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ISwapERC20Events::AuthorizeFilter(element) => element.fmt(f),
                ISwapERC20Events::CancelFilter(element) => element.fmt(f),
                ISwapERC20Events::OwnershipTransferredFilter(element) => element.fmt(f),
                ISwapERC20Events::RevokeFilter(element) => element.fmt(f),
                ISwapERC20Events::SetProtocolFeeFilter(element) => element.fmt(f),
                ISwapERC20Events::SetProtocolFeeLightFilter(element) => element.fmt(f),
                ISwapERC20Events::SetProtocolFeeWalletFilter(element) => element.fmt(f),
                ISwapERC20Events::SetRebateMaxFilter(element) => element.fmt(f),
                ISwapERC20Events::SetRebateScaleFilter(element) => element.fmt(f),
                ISwapERC20Events::SetStakingFilter(element) => element.fmt(f),
                ISwapERC20Events::SwapFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `FEE_DIVISOR` function with signature `FEE_DIVISOR()` and selector `[158, 147, 173, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "FEE_DIVISOR", abi = "FEE_DIVISOR()")]
    pub struct FeeDivisorCall;
    #[doc = "Container type for all input parameters for the `ORDER_TYPEHASH` function with signature `ORDER_TYPEHASH()` and selector `[249, 115, 162, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ORDER_TYPEHASH", abi = "ORDER_TYPEHASH()")]
    pub struct OrderTypehashCall;
    #[doc = "Container type for all input parameters for the `authorize` function with signature `authorize(address)` and selector `[182, 165, 215, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "authorize", abi = "authorize(address)")]
    pub struct AuthorizeCall {
        pub signer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `authorized` function with signature `authorized(address)` and selector `[185, 24, 22, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "authorized", abi = "authorized(address)")]
    pub struct AuthorizedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `calculateDiscount` function with signature `calculateDiscount(uint256,uint256)` and selector `[77, 42, 242, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "calculateDiscount", abi = "calculateDiscount(uint256,uint256)")]
    pub struct CalculateDiscountCall {
        pub staking_balance: ethers::core::types::U256,
        pub fee_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculateProtocolFee` function with signature `calculateProtocolFee(address,uint256)` and selector `[82, 197, 241, 245]`"]
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
        name = "calculateProtocolFee",
        abi = "calculateProtocolFee(address,uint256)"
    )]
    pub struct CalculateProtocolFeeCall {
        pub wallet: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancel` function with signature `cancel(uint256[])` and selector `[46, 52, 8, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancel", abi = "cancel(uint256[])")]
    pub struct CancelCall {
        pub nonces: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `check` function with signature `check(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[185, 203, 1, 176]`"]
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
        name = "check",
        abi = "check(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct CheckCall {
        pub sender_wallet: ethers::core::types::Address,
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
        pub signer: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `protocolFee` function with signature `protocolFee()` and selector `[176, 226, 30, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "protocolFee", abi = "protocolFee()")]
    pub struct ProtocolFeeCall;
    #[doc = "Container type for all input parameters for the `protocolFeeLight` function with signature `protocolFeeLight()` and selector `[244, 235, 198, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "protocolFeeLight", abi = "protocolFeeLight()")]
    pub struct ProtocolFeeLightCall;
    #[doc = "Container type for all input parameters for the `protocolFeeWallet` function with signature `protocolFeeWallet()` and selector `[203, 247, 198, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "protocolFeeWallet", abi = "protocolFeeWallet()")]
    pub struct ProtocolFeeWalletCall;
    #[doc = "Container type for all input parameters for the `rebateMax` function with signature `rebateMax()` and selector `[119, 15, 222, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rebateMax", abi = "rebateMax()")]
    pub struct RebateMaxCall;
    #[doc = "Container type for all input parameters for the `rebateScale` function with signature `rebateScale()` and selector `[219, 152, 92, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rebateScale", abi = "rebateScale()")]
    pub struct RebateScaleCall;
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
    #[doc = "Container type for all input parameters for the `revoke` function with signature `revoke()` and selector `[182, 84, 159, 117]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "revoke", abi = "revoke()")]
    pub struct RevokeCall;
    #[doc = "Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256)` and selector `[120, 125, 206, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256)")]
    pub struct SetProtocolFeeCall {
        pub protocol_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setProtocolFeeLight` function with signature `setProtocolFeeLight(uint256)` and selector `[191, 212, 229, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setProtocolFeeLight", abi = "setProtocolFeeLight(uint256)")]
    pub struct SetProtocolFeeLightCall {
        pub protocol_fee_light: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setProtocolFeeWallet` function with signature `setProtocolFeeWallet(address)` and selector `[124, 231, 133, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setProtocolFeeWallet", abi = "setProtocolFeeWallet(address)")]
    pub struct SetProtocolFeeWalletCall {
        pub protocol_fee_wallet: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setRebateMax` function with signature `setRebateMax(uint256)` and selector `[211, 30, 170, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRebateMax", abi = "setRebateMax(uint256)")]
    pub struct SetRebateMaxCall {
        pub rebate_max: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setRebateScale` function with signature `setRebateScale(uint256)` and selector `[233, 91, 119, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRebateScale", abi = "setRebateScale(uint256)")]
    pub struct SetRebateScaleCall {
        pub rebate_scale: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStaking` function with signature `setStaking(address)` and selector `[143, 243, 144, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStaking", abi = "setStaking(address)")]
    pub struct SetStakingCall {
        pub newstaking: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `staking` function with signature `staking()` and selector `[76, 240, 136, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "staking", abi = "staking()")]
    pub struct StakingCall;
    #[doc = "Container type for all input parameters for the `swap` function with signature `swap(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[152, 149, 96, 105]`"]
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
        abi = "swap(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SwapCall {
        pub recipient: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `swapAnySender` function with signature `swapAnySender(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[62, 177, 175, 36]`"]
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
        abi = "swapAnySender(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SwapAnySenderCall {
        pub recipient: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `swapLight` function with signature `swapLight(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[70, 228, 72, 13]`"]
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
        name = "swapLight",
        abi = "swapLight(uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SwapLightCall {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ISwapERC20Calls {
        DomainChainId(DomainChainIdCall),
        DomainName(DomainNameCall),
        DomainSeparator(DomainSeparatorCall),
        DomainTypehash(DomainTypehashCall),
        DomainVersion(DomainVersionCall),
        FeeDivisor(FeeDivisorCall),
        OrderTypehash(OrderTypehashCall),
        Authorize(AuthorizeCall),
        Authorized(AuthorizedCall),
        CalculateDiscount(CalculateDiscountCall),
        CalculateProtocolFee(CalculateProtocolFeeCall),
        Cancel(CancelCall),
        Check(CheckCall),
        GetChainId(GetChainIdCall),
        NonceUsed(NonceUsedCall),
        Owner(OwnerCall),
        ProtocolFee(ProtocolFeeCall),
        ProtocolFeeLight(ProtocolFeeLightCall),
        ProtocolFeeWallet(ProtocolFeeWalletCall),
        RebateMax(RebateMaxCall),
        RebateScale(RebateScaleCall),
        RenounceOwnership(RenounceOwnershipCall),
        Revoke(RevokeCall),
        SetProtocolFee(SetProtocolFeeCall),
        SetProtocolFeeLight(SetProtocolFeeLightCall),
        SetProtocolFeeWallet(SetProtocolFeeWalletCall),
        SetRebateMax(SetRebateMaxCall),
        SetRebateScale(SetRebateScaleCall),
        SetStaking(SetStakingCall),
        Staking(StakingCall),
        Swap(SwapCall),
        SwapAnySender(SwapAnySenderCall),
        SwapLight(SwapLightCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ISwapERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::DomainChainId(decoded));
            }
            if let Ok(decoded) =
                <DomainNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::DomainName(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::DomainVersion(decoded));
            }
            if let Ok(decoded) =
                <FeeDivisorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::FeeDivisor(decoded));
            }
            if let Ok(decoded) =
                <OrderTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::OrderTypehash(decoded));
            }
            if let Ok(decoded) =
                <AuthorizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Authorize(decoded));
            }
            if let Ok(decoded) =
                <AuthorizedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Authorized(decoded));
            }
            if let Ok(decoded) =
                <CalculateDiscountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::CalculateDiscount(decoded));
            }
            if let Ok(decoded) =
                <CalculateProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::CalculateProtocolFee(decoded));
            }
            if let Ok(decoded) = <CancelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Cancel(decoded));
            }
            if let Ok(decoded) = <CheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Check(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <NonceUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::NonceUsed(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::ProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeeLightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::ProtocolFeeLight(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeeWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::ProtocolFeeWallet(decoded));
            }
            if let Ok(decoded) =
                <RebateMaxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::RebateMax(decoded));
            }
            if let Ok(decoded) =
                <RebateScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::RebateScale(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RevokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Revoke(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeLightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetProtocolFeeLight(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetProtocolFeeWallet(decoded));
            }
            if let Ok(decoded) =
                <SetRebateMaxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetRebateMax(decoded));
            }
            if let Ok(decoded) =
                <SetRebateScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetRebateScale(decoded));
            }
            if let Ok(decoded) =
                <SetStakingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SetStaking(decoded));
            }
            if let Ok(decoded) =
                <StakingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::Staking(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ISwapERC20Calls::Swap(decoded));
            }
            if let Ok(decoded) =
                <SwapAnySenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SwapAnySender(decoded));
            }
            if let Ok(decoded) =
                <SwapLightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::SwapLight(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapERC20Calls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ISwapERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ISwapERC20Calls::DomainChainId(element) => element.encode(),
                ISwapERC20Calls::DomainName(element) => element.encode(),
                ISwapERC20Calls::DomainSeparator(element) => element.encode(),
                ISwapERC20Calls::DomainTypehash(element) => element.encode(),
                ISwapERC20Calls::DomainVersion(element) => element.encode(),
                ISwapERC20Calls::FeeDivisor(element) => element.encode(),
                ISwapERC20Calls::OrderTypehash(element) => element.encode(),
                ISwapERC20Calls::Authorize(element) => element.encode(),
                ISwapERC20Calls::Authorized(element) => element.encode(),
                ISwapERC20Calls::CalculateDiscount(element) => element.encode(),
                ISwapERC20Calls::CalculateProtocolFee(element) => element.encode(),
                ISwapERC20Calls::Cancel(element) => element.encode(),
                ISwapERC20Calls::Check(element) => element.encode(),
                ISwapERC20Calls::GetChainId(element) => element.encode(),
                ISwapERC20Calls::NonceUsed(element) => element.encode(),
                ISwapERC20Calls::Owner(element) => element.encode(),
                ISwapERC20Calls::ProtocolFee(element) => element.encode(),
                ISwapERC20Calls::ProtocolFeeLight(element) => element.encode(),
                ISwapERC20Calls::ProtocolFeeWallet(element) => element.encode(),
                ISwapERC20Calls::RebateMax(element) => element.encode(),
                ISwapERC20Calls::RebateScale(element) => element.encode(),
                ISwapERC20Calls::RenounceOwnership(element) => element.encode(),
                ISwapERC20Calls::Revoke(element) => element.encode(),
                ISwapERC20Calls::SetProtocolFee(element) => element.encode(),
                ISwapERC20Calls::SetProtocolFeeLight(element) => element.encode(),
                ISwapERC20Calls::SetProtocolFeeWallet(element) => element.encode(),
                ISwapERC20Calls::SetRebateMax(element) => element.encode(),
                ISwapERC20Calls::SetRebateScale(element) => element.encode(),
                ISwapERC20Calls::SetStaking(element) => element.encode(),
                ISwapERC20Calls::Staking(element) => element.encode(),
                ISwapERC20Calls::Swap(element) => element.encode(),
                ISwapERC20Calls::SwapAnySender(element) => element.encode(),
                ISwapERC20Calls::SwapLight(element) => element.encode(),
                ISwapERC20Calls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ISwapERC20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ISwapERC20Calls::DomainChainId(element) => element.fmt(f),
                ISwapERC20Calls::DomainName(element) => element.fmt(f),
                ISwapERC20Calls::DomainSeparator(element) => element.fmt(f),
                ISwapERC20Calls::DomainTypehash(element) => element.fmt(f),
                ISwapERC20Calls::DomainVersion(element) => element.fmt(f),
                ISwapERC20Calls::FeeDivisor(element) => element.fmt(f),
                ISwapERC20Calls::OrderTypehash(element) => element.fmt(f),
                ISwapERC20Calls::Authorize(element) => element.fmt(f),
                ISwapERC20Calls::Authorized(element) => element.fmt(f),
                ISwapERC20Calls::CalculateDiscount(element) => element.fmt(f),
                ISwapERC20Calls::CalculateProtocolFee(element) => element.fmt(f),
                ISwapERC20Calls::Cancel(element) => element.fmt(f),
                ISwapERC20Calls::Check(element) => element.fmt(f),
                ISwapERC20Calls::GetChainId(element) => element.fmt(f),
                ISwapERC20Calls::NonceUsed(element) => element.fmt(f),
                ISwapERC20Calls::Owner(element) => element.fmt(f),
                ISwapERC20Calls::ProtocolFee(element) => element.fmt(f),
                ISwapERC20Calls::ProtocolFeeLight(element) => element.fmt(f),
                ISwapERC20Calls::ProtocolFeeWallet(element) => element.fmt(f),
                ISwapERC20Calls::RebateMax(element) => element.fmt(f),
                ISwapERC20Calls::RebateScale(element) => element.fmt(f),
                ISwapERC20Calls::RenounceOwnership(element) => element.fmt(f),
                ISwapERC20Calls::Revoke(element) => element.fmt(f),
                ISwapERC20Calls::SetProtocolFee(element) => element.fmt(f),
                ISwapERC20Calls::SetProtocolFeeLight(element) => element.fmt(f),
                ISwapERC20Calls::SetProtocolFeeWallet(element) => element.fmt(f),
                ISwapERC20Calls::SetRebateMax(element) => element.fmt(f),
                ISwapERC20Calls::SetRebateScale(element) => element.fmt(f),
                ISwapERC20Calls::SetStaking(element) => element.fmt(f),
                ISwapERC20Calls::Staking(element) => element.fmt(f),
                ISwapERC20Calls::Swap(element) => element.fmt(f),
                ISwapERC20Calls::SwapAnySender(element) => element.fmt(f),
                ISwapERC20Calls::SwapLight(element) => element.fmt(f),
                ISwapERC20Calls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainChainIdCall> for ISwapERC20Calls {
        fn from(var: DomainChainIdCall) -> Self {
            ISwapERC20Calls::DomainChainId(var)
        }
    }
    impl ::std::convert::From<DomainNameCall> for ISwapERC20Calls {
        fn from(var: DomainNameCall) -> Self {
            ISwapERC20Calls::DomainName(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for ISwapERC20Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            ISwapERC20Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<DomainTypehashCall> for ISwapERC20Calls {
        fn from(var: DomainTypehashCall) -> Self {
            ISwapERC20Calls::DomainTypehash(var)
        }
    }
    impl ::std::convert::From<DomainVersionCall> for ISwapERC20Calls {
        fn from(var: DomainVersionCall) -> Self {
            ISwapERC20Calls::DomainVersion(var)
        }
    }
    impl ::std::convert::From<FeeDivisorCall> for ISwapERC20Calls {
        fn from(var: FeeDivisorCall) -> Self {
            ISwapERC20Calls::FeeDivisor(var)
        }
    }
    impl ::std::convert::From<OrderTypehashCall> for ISwapERC20Calls {
        fn from(var: OrderTypehashCall) -> Self {
            ISwapERC20Calls::OrderTypehash(var)
        }
    }
    impl ::std::convert::From<AuthorizeCall> for ISwapERC20Calls {
        fn from(var: AuthorizeCall) -> Self {
            ISwapERC20Calls::Authorize(var)
        }
    }
    impl ::std::convert::From<AuthorizedCall> for ISwapERC20Calls {
        fn from(var: AuthorizedCall) -> Self {
            ISwapERC20Calls::Authorized(var)
        }
    }
    impl ::std::convert::From<CalculateDiscountCall> for ISwapERC20Calls {
        fn from(var: CalculateDiscountCall) -> Self {
            ISwapERC20Calls::CalculateDiscount(var)
        }
    }
    impl ::std::convert::From<CalculateProtocolFeeCall> for ISwapERC20Calls {
        fn from(var: CalculateProtocolFeeCall) -> Self {
            ISwapERC20Calls::CalculateProtocolFee(var)
        }
    }
    impl ::std::convert::From<CancelCall> for ISwapERC20Calls {
        fn from(var: CancelCall) -> Self {
            ISwapERC20Calls::Cancel(var)
        }
    }
    impl ::std::convert::From<CheckCall> for ISwapERC20Calls {
        fn from(var: CheckCall) -> Self {
            ISwapERC20Calls::Check(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for ISwapERC20Calls {
        fn from(var: GetChainIdCall) -> Self {
            ISwapERC20Calls::GetChainId(var)
        }
    }
    impl ::std::convert::From<NonceUsedCall> for ISwapERC20Calls {
        fn from(var: NonceUsedCall) -> Self {
            ISwapERC20Calls::NonceUsed(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ISwapERC20Calls {
        fn from(var: OwnerCall) -> Self {
            ISwapERC20Calls::Owner(var)
        }
    }
    impl ::std::convert::From<ProtocolFeeCall> for ISwapERC20Calls {
        fn from(var: ProtocolFeeCall) -> Self {
            ISwapERC20Calls::ProtocolFee(var)
        }
    }
    impl ::std::convert::From<ProtocolFeeLightCall> for ISwapERC20Calls {
        fn from(var: ProtocolFeeLightCall) -> Self {
            ISwapERC20Calls::ProtocolFeeLight(var)
        }
    }
    impl ::std::convert::From<ProtocolFeeWalletCall> for ISwapERC20Calls {
        fn from(var: ProtocolFeeWalletCall) -> Self {
            ISwapERC20Calls::ProtocolFeeWallet(var)
        }
    }
    impl ::std::convert::From<RebateMaxCall> for ISwapERC20Calls {
        fn from(var: RebateMaxCall) -> Self {
            ISwapERC20Calls::RebateMax(var)
        }
    }
    impl ::std::convert::From<RebateScaleCall> for ISwapERC20Calls {
        fn from(var: RebateScaleCall) -> Self {
            ISwapERC20Calls::RebateScale(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ISwapERC20Calls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ISwapERC20Calls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RevokeCall> for ISwapERC20Calls {
        fn from(var: RevokeCall) -> Self {
            ISwapERC20Calls::Revoke(var)
        }
    }
    impl ::std::convert::From<SetProtocolFeeCall> for ISwapERC20Calls {
        fn from(var: SetProtocolFeeCall) -> Self {
            ISwapERC20Calls::SetProtocolFee(var)
        }
    }
    impl ::std::convert::From<SetProtocolFeeLightCall> for ISwapERC20Calls {
        fn from(var: SetProtocolFeeLightCall) -> Self {
            ISwapERC20Calls::SetProtocolFeeLight(var)
        }
    }
    impl ::std::convert::From<SetProtocolFeeWalletCall> for ISwapERC20Calls {
        fn from(var: SetProtocolFeeWalletCall) -> Self {
            ISwapERC20Calls::SetProtocolFeeWallet(var)
        }
    }
    impl ::std::convert::From<SetRebateMaxCall> for ISwapERC20Calls {
        fn from(var: SetRebateMaxCall) -> Self {
            ISwapERC20Calls::SetRebateMax(var)
        }
    }
    impl ::std::convert::From<SetRebateScaleCall> for ISwapERC20Calls {
        fn from(var: SetRebateScaleCall) -> Self {
            ISwapERC20Calls::SetRebateScale(var)
        }
    }
    impl ::std::convert::From<SetStakingCall> for ISwapERC20Calls {
        fn from(var: SetStakingCall) -> Self {
            ISwapERC20Calls::SetStaking(var)
        }
    }
    impl ::std::convert::From<StakingCall> for ISwapERC20Calls {
        fn from(var: StakingCall) -> Self {
            ISwapERC20Calls::Staking(var)
        }
    }
    impl ::std::convert::From<SwapCall> for ISwapERC20Calls {
        fn from(var: SwapCall) -> Self {
            ISwapERC20Calls::Swap(var)
        }
    }
    impl ::std::convert::From<SwapAnySenderCall> for ISwapERC20Calls {
        fn from(var: SwapAnySenderCall) -> Self {
            ISwapERC20Calls::SwapAnySender(var)
        }
    }
    impl ::std::convert::From<SwapLightCall> for ISwapERC20Calls {
        fn from(var: SwapLightCall) -> Self {
            ISwapERC20Calls::SwapLight(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ISwapERC20Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            ISwapERC20Calls::TransferOwnership(var)
        }
    }
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
    #[doc = "Container type for all return fields from the `FEE_DIVISOR` function with signature `FEE_DIVISOR()` and selector `[158, 147, 173, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeDivisorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ORDER_TYPEHASH` function with signature `ORDER_TYPEHASH()` and selector `[249, 115, 162, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OrderTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `authorized` function with signature `authorized(address)` and selector `[185, 24, 22, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AuthorizedReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `calculateDiscount` function with signature `calculateDiscount(uint256,uint256)` and selector `[77, 42, 242, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateDiscountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `calculateProtocolFee` function with signature `calculateProtocolFee(address,uint256)` and selector `[82, 197, 241, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateProtocolFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `check` function with signature `check(address,uint256,uint256,address,address,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[185, 203, 1, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CheckReturn(pub ethers::core::types::U256, pub ::std::vec::Vec<[u8; 32]>);
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
    #[doc = "Container type for all return fields from the `protocolFee` function with signature `protocolFee()` and selector `[176, 226, 30, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProtocolFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `protocolFeeLight` function with signature `protocolFeeLight()` and selector `[244, 235, 198, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProtocolFeeLightReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `protocolFeeWallet` function with signature `protocolFeeWallet()` and selector `[203, 247, 198, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProtocolFeeWalletReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `rebateMax` function with signature `rebateMax()` and selector `[119, 15, 222, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RebateMaxReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `rebateScale` function with signature `rebateScale()` and selector `[219, 152, 92, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RebateScaleReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `staking` function with signature `staking()` and selector `[76, 240, 136, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StakingReturn(pub ethers::core::types::Address);
}
