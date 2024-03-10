use ethers::providers::ProviderError;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::task;

use crate::{
    clients::{blockchain_client::BlockchainClientTrait, redis_client::RedisClientTrait},
    config::Config,
};

#[derive(Clone)]
pub struct ChainSynchronizer<B: BlockchainClientTrait, R: RedisClientTrait> {
    blockchain_client: B,
    redis_client: R,
    config: Config,
}

impl<B: BlockchainClientTrait, R: RedisClientTrait> ChainSynchronizer<B, R> {
    pub fn new(blockchain_client: B, redis_client: R, config: Config) -> Self {
        Self {
            blockchain_client,
            redis_client,
            config,
        }
    }

    pub async fn sync(&self, start_block: u64, end_block: u64) {
        for block_number in start_block..=end_block {
            if let Ok(Some(block)) = self
                .blockchain_client
                .get_block_with_txs(block_number)
                .await
            {
                println!(
                    "Block Number {}: {} transactions",
                    block.number.unwrap(),
                    block.transactions.len()
                );

                let mut futures = FuturesUnordered::new();
                for transaction in block.transactions {
                    let blockchain_client = self.blockchain_client.clone();
                    let redis_client = self.redis_client.clone();
                    futures.push(task::spawn(async move {
                        if let Ok(Some(receipt)) = blockchain_client
                            .get_transaction_receipt(transaction.hash)
                            .await
                        {
                            let logs = receipt.logs;
                            if let Err(e) = redis_client
                                .send_logs("ASSETS_INDEXER_STREAM".to_string(), logs)
                                .await
                            {
                                println!(
                                    "Error sending logs for transaction hash {} error {}",
                                    transaction.hash, e
                                )
                            }
                        }
                    }));

                    if futures.len() >= self.config.num_workers {
                        futures.next().await;
                    }
                }

                while futures.next().await.is_some() {}
            }
        }
    }

    pub fn start_block(&self) -> u64 {
        self.config.start_block.unwrap_or(0)
    }

    pub async fn end_block(&self) -> Result<u64, ProviderError> {
        Ok(self
            .config
            .end_block
            .unwrap_or(self.blockchain_client.get_block_number().await?.as_u64()))
    }
}
