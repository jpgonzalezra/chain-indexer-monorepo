use std::time::Instant;

use ethers::{
    providers::ProviderError,
    types::{Block as EthersBlock, Transaction},
};
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::task;

use crate::{
    clients::{blockchain_client::BlockchainClientTrait, redis_client::RedisClientTrait},
    config::Config,
};

use super::repositories::block::{Block, BlockRepositoryTrait};

#[derive(Clone)]
pub struct ChainSynchronizer<
    B: BlockchainClientTrait,
    R: RedisClientTrait,
    E: BlockRepositoryTrait,
> {
    blockchain_client: B,
    redis_client: R,
    block_repository: E,
    config: Config,
}

impl<B: BlockchainClientTrait, R: RedisClientTrait, E: BlockRepositoryTrait>
    ChainSynchronizer<B, R, E>
{
    pub fn new(
        blockchain_client: B,
        redis_client: R,
        block_repository: E,
        config: Config,
    ) -> Self {
        Self {
            blockchain_client,
            redis_client,
            block_repository,
            config,
        }
    }

    pub async fn sync_missing_blocks(&self, blocks: Vec<u64>) {
        self.process_blocks(blocks.into_iter()).await;
    }

    pub async fn sync(&self, start_block: u64, end_block: u64) {
        self.process_blocks(start_block..=end_block).await;
    }

    async fn process_blocks(
        &self,
        block_numbers: impl Iterator<Item = u64> + Send + 'static,
    ) {
        let mut futures = FuturesUnordered::new();

        for block_number in block_numbers {
            let self_clone = self.clone();
            futures.push(task::spawn(async move {
                if let Ok(Some(block)) = self_clone
                    .blockchain_client
                    .get_block_with_txs(block_number)
                    .await
                {
                    self_clone.process_block(block.clone()).await;
                }
            }));

            if futures.len() >= self.config.num_workers {
                futures.next().await;
            }
        }

        while futures.next().await.is_some() {}
    }

    async fn process_block(&self, block: EthersBlock<Transaction>) {
        let start_time = Instant::now();

        let mut futures = FuturesUnordered::new();
        for transaction in block.transactions {
            let blockchain_client = self.blockchain_client.clone();
            let redis_client = self.redis_client.clone();
            let stream_key = self.config.redis_config.stream_key.clone();
            futures.push(task::spawn(async move {
                if let Ok(Some(receipt)) = blockchain_client
                    .get_transaction_receipt(transaction.hash)
                    .await
                {
                    let logs = receipt.logs;
                    if let Err(e) = redis_client.send_logs(stream_key, logs).await {
                        tracing::error!(
                            "Error sending logs for transaction hash {} error {}",
                            transaction.hash,
                            e
                        )
                    }
                }
            }));

            if futures.len() >= self.config.num_workers {
                futures.next().await;
            }
        }

        while futures.next().await.is_some() {}

        let block_number = block.number.unwrap().as_u64();
        match self
            .block_repository
            .insert_block(Block {
                block_number,
                chain_id: self.config.chain.id,
            })
            .await
        {
            Ok(_) => {
                tracing::debug!("Blocks inserted successfully");
            }
            Err(error) => {
                tracing::error!("Error inserting blocks: {:?}", error);
            }
        }

        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);

        tracing::info!(
            "Block number {:?} processed in {:?}.",
            block_number,
            duration
        );
    }

    pub fn start_block(&self) -> u64 {
        self.config.start_block.unwrap_or(0)
    }

    pub async fn missing_blocks(&self) -> Result<Vec<u64>, sqlx::Error> {
        let indexed_blocks = match self.block_repository.get_indexed_blocks().await {
            Ok(result) => result,
            Err(err) => {
                tracing::error!("Error on retrieve missing block operation: {}.", err);
                Vec::new()
            }
        };

        if indexed_blocks.is_empty() {
            return Ok(Vec::new());
        }

        let end_block = self.end_block().await.unwrap_or(self.start_block());
        let full_block_range: Vec<u64> = (self.start_block()..end_block).collect();

        let missing_blocks: Vec<u64> = full_block_range
            .into_iter()
            .filter(|block| !indexed_blocks.contains(block))
            .collect();
        Ok(missing_blocks)
    }

    pub async fn end_block(&self) -> Result<u64, ProviderError> {
        Ok(self
            .config
            .end_block
            .unwrap_or(self.blockchain_client.get_block_number().await?))
    }
}
