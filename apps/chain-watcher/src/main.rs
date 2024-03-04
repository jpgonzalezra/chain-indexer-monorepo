use common::redis::redis_client_factory;
use common::types::SummaryLog;
use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use redis::AsyncCommands;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut redis_conn = redis_client_factory()
        .get_async_connection()
        .await
        .expect("Error on acquiring redis connection");

    let rpc_url = env::var("ETHEREUM_RPC_URL").unwrap();
    let http_provider = Provider::<Http>::try_from(rpc_url)?;

    let provider = Arc::new(http_provider);
    let start_block = 19249009;
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
                let tx_receipt =
                    provider.get_transaction_receipt(transaction.hash).await?;
                if let Some(receipt) = tx_receipt {
                    for log in receipt.logs {
                        let summary_log: SummaryLog = log.into();
                        let message = serde_json::to_string(&summary_log).unwrap();
                        let _: () = redis_conn
                            .xadd("ASSETS_INDEXER_STREAM", "*", &[("message", &message)])
                            .await
                            .unwrap();
                    }
                }
            }
        }
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
