use ethers::prelude::*;
use crate::utils::types::UnsignedClaim;

fn create_claim(nonce: Option<U256>, expiry: Option<U256>, participant: Option<Address>, score: Option<U256>, use_default: bool) -> UnsignedClaim{
    if use_default {
        return  UnsignedClaim::default()
    }
    UnsignedClaim{nonce: nonce.unwrap(), expiry: expiry.unwrap(), participant: participant.unwrap(), score: score.unwrap()}
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_create_claim() {
       let claim = create_claim(None, None, None, None, true);
        let default_claim = UnsignedClaim::default();
        assert_eq!(claim.nonce, default_claim.nonce );
        assert_eq!(claim.expiry, default_claim.expiry );
        assert_eq!(claim.participant, default_claim.participant );
        assert_eq!(claim.score, default_claim.score );
    }
}
