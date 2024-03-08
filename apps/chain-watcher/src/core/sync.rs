use std::sync::Arc;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use common::{redis::redis_pool_factory, types::SummaryLog};
use ethers::providers::{Http, Middleware, Provider};
use futures::stream::{FuturesUnordered, StreamExt};
use redis::AsyncCommands;
use tokio::task;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ChainSynchronizer {
    pub redis_pool: Pool<RedisConnectionManager>,
    pub provider: Arc<Provider<Http>>,
    pub num_workers: usize,
}

impl ChainSynchronizer {
    pub async fn new(config: Config) -> Self {
        let redis_config = config.redis_config;
        let redis_pool = redis_pool_factory(
            redis_config.host,
            redis_config.port,
            redis_config.password,
            redis_config.db,
        )
        .await
        .expect("Error on acquiring redis connection.");

        let http_provider = Provider::<Http>::try_from(config.rpc)
            .expect("Error on provider http creation.");
        let provider = Arc::new(http_provider);

        ChainSynchronizer {
            redis_pool,
            provider,
            num_workers: config.num_workers,
        }
    }

    pub async fn sync(&self, start_block: u64, end_block: u64) {
        for block_number in start_block..=end_block {
            if let Ok(Some(block)) = self.provider.get_block_with_txs(block_number).await
            {
                println!(
                    "Block Number {}: {} transactions",
                    block.number.unwrap(),
                    block.transactions.len()
                );

                let mut futures = FuturesUnordered::new();
                for transaction in block.transactions {
                    let provider = self.provider.clone();
                    let redis_pool_clone: Pool<RedisConnectionManager> =
                        self.redis_pool.clone();

                    futures.push(task::spawn(async move {
                        if let Ok(Some(receipt)) =
                            provider.get_transaction_receipt(transaction.hash).await
                        {
                            let logs = receipt.logs;
                            if logs.is_empty() {
                                return;
                            }
                            let mut conn = redis_pool_clone
                                .get()
                                .await
                                .expect("Pool connection Error");

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

                    if futures.len() >= self.num_workers {
                        futures.next().await;
                    }
                }

                while futures.next().await.is_some() {}
            }
        }
    }
}
