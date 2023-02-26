use crate::utils::types::{OrderParty, UnsignedOrder};
use ethers::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order() {
        let order = create_order(None, None, None, None, None, None, true);
        let default_order = UnsignedOrder::default();
        assert_eq!(order.nonce, default_order.nonce);
        assert_eq!(order.expiry, default_order.expiry);
        assert_eq!(order.protocol_fee, default_order.protocol_fee);
    }
}
