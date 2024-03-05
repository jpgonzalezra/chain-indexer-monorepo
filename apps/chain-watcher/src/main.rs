use common::redis::redis_pool_factory;
use common::types::SummaryLog;
use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use futures::stream::{FuturesUnordered, StreamExt};
use redis::AsyncCommands;
use std::{env, sync::Arc};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = redis_pool_factory()
        .await
        .expect("Error on acquiring redis connection");
    let rpc_url = env::var("ETHEREUM_RPC_URL").unwrap();
    let http_provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(http_provider);

    let start_block = 19249009;
    let end_block = provider.get_block_number().await?;
    println!("Indexing from block {} to block {}", start_block, end_block);

    let cpu_cores = num_cpus::get();
    println!("Using {} workers.", cpu_cores);

    for block_number in start_block..=end_block.as_u64() {
        let provider = provider.clone();

        if let Ok(Some(block)) = provider.get_block_with_txs(block_number).await {
            println!(
                "Block Number {}: {} transactions",
                block.number.unwrap(),
                block.transactions.len()
            );

            let mut futures = FuturesUnordered::new();
            for transaction in block.transactions {
                let provider = provider.clone();
                let pool_clone = pool.clone();

                futures.push(task::spawn(async move {
                    if let Ok(Some(receipt)) =
                        provider.get_transaction_receipt(transaction.hash).await
                    {
                        let logs = receipt.logs;
                        if logs.is_empty() {
                            return;
                        }
                        let mut conn =
                            pool_clone.get().await.expect("Pool connection Error");

                        let summary_logs: Vec<SummaryLog> =
                            logs.iter().map(|log| log.clone().into()).collect();
                        let message: String =
                            serde_json::to_string(&summary_logs).unwrap();

                        let _: () = conn
                            .xadd("ASSETS_INDEXER_STREAM", "*", &[("message", &message)])
                            .await
                            .expect("Error sending message");
                    }
                }));

                if futures.len() >= cpu_cores {
                    futures.next().await;
                }
            }

            while futures.next().await.is_some() {}
        }
    }

    Ok(())
}
