use airswap_rs::contracts::addresses::try_contract;
use airswap_rs::pool::Pool;

use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .unwrap();
    let signer = LocalWallet::new(&mut rand::thread_rng());
    let signer_middleware = SignerMiddleware::new(provider, signer);

    let client = Arc::new(signer_middleware);

    let pool_address = try_contract("POOL")
        .unwrap()
        .address(Chain::Mainnet)
        .unwrap();

    let pool = Pool::new(client, pool_address);

    let scale = pool.scale().await?;
    println!("The pool contract scale is - {}", scale);

    Ok(())
}
