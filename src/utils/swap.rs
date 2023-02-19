
use crate::utils::types::{OrderParty, UnsignedOrder};
use ethers::prelude::*;
// use ethers::{
//     types::{Address, Bytes, NameOrAddress},
//     signers::LocalWallet,
//     utils::{keccak256, TypedData},
// };

pub fn create_order(
    nonce: Option<U256>,
    expiry: Option<U256>,
    protocol_fee: Option<U256>,
    signer: Option<OrderParty>,
    sender: Option<OrderParty>,
    affiliate: Option<OrderParty>,
    use_default: bool,
) -> UnsignedOrder {
    if use_default {
        return UnsignedOrder::default();
    }

    UnsignedOrder {
        nonce: nonce.unwrap(),
        expiry: expiry.unwrap(),
        protocol_fee: protocol_fee.unwrap(),
        signer: signer.unwrap(),
        sender: sender.unwrap(),
        affiliate: affiliate.unwrap(),
    }
}

// pub fn create_order_signature(unsigned: UnsignedOrder, signer: LocalWallet, swap_contract: Address, chain_id: U256) -> Signature{
//     Signature::default()
// }

// export async function createOrderSignature(
//     unsignedOrder: UnsignedOrder,
//     signer: ethers.VoidSigner | string,
//     swapContract: string,
//     chainId: number
//   ): Promise<Signature> {

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order() {
        let claim = create_order(None, None, None, None, None, None, true);
        let default_order = UnsignedOrder::default();
        assert_eq!(claim.nonce, default_order.nonce);
        assert_eq!(claim.expiry, default_order.expiry);
        assert_eq!(claim.protocol_fee, default_order.protocol_fee);
    }
}
