use airswap_rs::prelude::*;

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create order
    let order = create_order(None, None, None, None, None, None, true);
    println!(
        "Default Order: Nonce {},\n Expiry {},\n Protocol Fee {},\n Signer {},\n Sender {},\n Affiliate {}",
        order.nonce,
        order.expiry,
        order.protocol_fee,
        order.signer.wallet,
        order.sender.wallet,
        order.affiliate.wallet
    );

    // Create claim
    let claim = create_claim(None, None, None, None, true);
    println!(
        "Default Claim: Nonce {},\n Expiry {},\n Participant {},\n Score {}",
        claim.nonce, claim.expiry, claim.participant, claim.score
    );

    Ok(())
}
