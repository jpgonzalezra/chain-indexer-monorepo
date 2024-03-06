pub mod config;

use common::redis::redis_pool_factory;
use common::types::SummaryLog;
use config::Config;
use ethers::providers::{Http, Middleware, Provider};
use futures::stream::{FuturesUnordered, StreamExt};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let redis_config = config.redis_config;
    let pool = redis_pool_factory(
        redis_config.host,
        redis_config.port,
        redis_config.password,
        redis_config.db,
    )
    .await
    .expect("Error on acquiring redis connection");

    let rpc_url = config.rpcs[0].clone();
    let http_provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(http_provider);

    let cpu_cores = num_cpus::get();
    println!("Using {} workers.", cpu_cores);

    loop {
        let mut start_block = config.start_block.unwrap_or(0);

        let end_block = config
            .end_block
            .unwrap_or(provider.get_block_number().await?.as_u64());

        println!("Indexing from block {} to block {}", start_block, end_block);

        for block_number in start_block..=end_block {
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
                                .xadd(
                                    "ASSETS_INDEXER_STREAM",
                                    "*",
                                    &[("message", &message)],
                                )
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
        start_block = end_block + 1;
        println!(
            "Updating start and end block, from {} to {}",
            start_block, end_block
        );
    }
}
