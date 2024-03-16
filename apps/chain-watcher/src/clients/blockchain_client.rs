use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    providers::{Http, Middleware, Provider, ProviderError},
    types::{Block, Transaction, TransactionReceipt, H256},
};

#[async_trait]
pub trait BlockchainClientTrait: Clone + Send + Sync + 'static {
    async fn get_block_with_txs(
        &self,
        block_number: u64,
    ) -> Result<Option<Block<Transaction>>, ProviderError>;
    async fn get_transaction_receipt(
        &self,
        tx_hash: H256,
    ) -> Result<Option<TransactionReceipt>, ProviderError>;
    async fn get_block_number(&self) -> Result<u64, ProviderError>;
}

#[derive(Clone)]
pub struct BlockchainClient {
    pub provider: Arc<Provider<Http>>,
}

#[async_trait]
impl BlockchainClientTrait for BlockchainClient {
    async fn get_block_with_txs(
        &self,
        block_number: u64,
    ) -> Result<Option<Block<Transaction>>, ProviderError> {
        self.provider.clone().get_block_with_txs(block_number).await
    }

    async fn get_transaction_receipt(
        &self,
        tx_hash: H256,
    ) -> Result<Option<TransactionReceipt>, ProviderError> {
        self.provider.clone().get_transaction_receipt(tx_hash).await
    }

    async fn get_block_number(&self) -> Result<u64, ProviderError> {
        let result = self.provider.clone().get_block_number().await?;
        Ok(result.as_u64())
    }
}
