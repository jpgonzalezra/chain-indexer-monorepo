use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{transaction, Transaction};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let rpc_url = env::var("ETHEREUM_RPC_URL").unwrap();
    let http_provider = Provider::<Http>::try_from(rpc_url)?;

    let provider = Arc::new(http_provider);
    let start_block = 0;
    let end_block = provider.get_block_number().await?;

    println!("Indexing from block {} to block {}", start_block, end_block);

    for block_number in start_block..=end_block.as_u64() {
        let block = provider.get_block_with_txs(block_number).await?;

        if let Some(block) = block {
            println!(
                "Block Number {}: {} transactions",
                block.number.unwrap(),
                block.transactions.len()
            );

            for transaction in block.transactions {
                println!("Transaction: {:?}", transaction);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
