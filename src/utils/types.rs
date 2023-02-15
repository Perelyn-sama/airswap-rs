use crate::common::constants::{ADDRESS_ZERO, SECONDS_IN_DAY};
use chrono::prelude::*;
use ethers::prelude::*;

pub struct Signature {
    v: u8,
    r: [u8; 32],
    s: [u8; 32],
}

pub struct UnsignedOrderERC20 {
    nonce: U256,
    expiry: U256,
    signer_wallet: Address,
    signer_token: Address,
    signer_amount: U256,
    protocol_fee: U256,
    sender_wallet: Address,
    sender_token: Address,
    sender_amount: U256,
}

pub struct OrderERC20 {
    nonce: U256,
    expiry: U256,
    signer_wallet: Address,
    signer_token: Address,
    signer_amount: U256,
    sender_token: Address,
    sender_amount: Address,
    signature: Signature,
}

pub struct Settlement {
    chain_id: U256,
    swap_contract: Address,
}

pub struct FullOrderERC20 {
    unsigned_order_erc20: UnsignedOrderERC20,
    signature: Signature,
    settlement: Settlement,
}

pub struct UnsignedOrder {
    pub nonce: U256,
    pub expiry: U256,
    pub protocol_fee: U256,
    pub signer: OrderParty,
    pub sender: OrderParty,
    pub affiliate: OrderParty,
}

impl Default for UnsignedOrder {
    fn default() -> Self {
        let unix_timestamp = Utc::now().timestamp();
        Self {
            nonce: U256::from(unix_timestamp),
            expiry: U256::from(unix_timestamp / 1000 + SECONDS_IN_DAY),
            protocol_fee: U256::from(0),
            signer: OrderParty::default(),
            sender: OrderParty::default(),
            affiliate: OrderParty::default(),
        }
    }
}

pub struct Order {
    unsigned_order: UnsignedOrder,
    signature: Signature,
}

pub struct OrderParty {
    wallet: Address,
    token: Address,
    // identify kind variations and use them to make an enum
    kind: String,
    id: U256,
    amount: U256,
}

pub struct UnsignedClaim {
    pub nonce: U256,
    pub expiry: U256,
    pub participant: Address,
    pub score: U256,
}

impl Default for UnsignedClaim {
    fn default() -> Self {
        let unix_timestamp = Utc::now().timestamp();
        Self {
            nonce: U256::from(unix_timestamp),
            expiry: U256::from(unix_timestamp + 60),
            participant: ADDRESS_ZERO.parse().unwrap(),
            score: U256::from(0),
        }
    }
}

pub struct Claim {
    nonce: U256,
    expiry: U256,
    participant: Address,
    score: U256,
    signature: Signature,
}

pub struct Token {
    address: Address,
    symbol: String,
    decimals: U256,
}

pub struct LocatorResult {
    locators: Vec<String>,
    scores: Vec<String>,
    next_cursor: String,
}
//
// export const EIP712SwapERC20 = {
//     EIP712Domain: [
//     { name: 'name', type: 'string' },
//     { name: 'version', type: 'string' },
//     { name: 'chainId', type: 'uint256' },
//     { name: 'verifyingContract', type: 'address' },
//     ],
//     OrderERC20: [
//     { name: 'nonce', type: 'uint256' },
//     { name: 'expiry', type: 'uint256' },
//     { name: 'signer_wallet', type: 'address' },
//     { name: 'signerToken', type: 'address' },
//     { name: 'signerAmount', type: 'uint256' },
//     { name: 'protocolFee', type: 'uint256' },
//     { name: 'senderWallet', type: 'address' },
//     { name: 'senderToken', type: 'address' },
//     { name: 'senderAmount', type: 'uint256' },
//     ],
// }
//
// export const EIP712Swap = {
//     EIP712Domain: [
//     { name: 'name', type: 'string' },
//     { name: 'version', type: 'string' },
//     { name: 'chainId', type: 'uint256' },
//     { name: 'verifyingContract', type: 'address' },
//     ],
//     Order: [
//     { name: 'nonce', type: 'uint256' },
//     { name: 'expiry', type: 'uint256' },
//     { name: 'protocolFee', type: 'uint256' },
//     { name: 'signer', type: 'Party' },
//     { name: 'sender', type: 'Party' },
//     { name: 'affiliate', type: 'Party' },
//     ],
//     Party: [
//     { name: 'wallet', type: 'address' },
//     { name: 'token', type: 'address' },
//     { name: 'kind', type: 'bytes4' },
//     { name: 'id', type: 'uint256' },
//     { name: 'amount', type: 'uint256' },
//     ],
// }
//
// export const EIP712Claim = {
//     EIP712Domain: [
//     { name: 'name', type: 'string' },
//     { name: 'version', type: 'string' },
//     { name: 'chainId', type: 'uint256' },
//     { name: 'verifyingContract', type: 'address' },
//     ],
//     Claim: [
//     { name: 'nonce', type: 'uint256' },
//     { name: 'expiry', type: 'uint256' },
//     { name: 'participant', type: 'address' },
//     { name: 'score', type: 'uint256' },
//     ],
// }
//
// export { TokenInfo } from '@uniswap/token-lists'
//
// export type Levels = [string, string][]
// export type Formula = string
//
// type LevelsOrFomulae =
// | {
// bid: Levels
// ask: Levels
// }
// | {
// bid: Formula
// ask: Formula
// }
//
// export type Pricing = {
// baseToken: string
// quoteToken: string
// minimum?: string
// } & LevelsOrFomulae
