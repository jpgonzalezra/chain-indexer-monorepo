use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{FromRow, PgPool};

use crate::config::ChainConfig;

#[derive(Debug, FromRow)]
pub struct BlockNumber {
    number_block: i64,
}

#[async_trait]
pub trait BlockRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(database_pool: Arc<PgPool>, chain_config: ChainConfig) -> Self;
    async fn get_indexed_blocks(&self) -> Result<Vec<u64>, sqlx::Error>;
}

#[derive(Clone)]
pub struct BlockRepository {
    pub database_pool: Arc<PgPool>,
    pub chain_config: ChainConfig,
}

#[async_trait]
impl BlockRepositoryTrait for BlockRepository {
    fn new(database_pool: Arc<PgPool>, chain_config: ChainConfig) -> Self {
        Self {
            database_pool,
            chain_config,
        }
    }

    async fn get_indexed_blocks(&self) -> Result<Vec<u64>, sqlx::Error> {
        let pool = self.database_pool.clone();
        let result = sqlx::query_as::<_, BlockNumber>(
            "SELECT number_block FROM Block WHERE chain_id = $1",
        )
        .bind(self.chain_config.id as i32)
        .fetch_all(&*pool)
        .await?
        .into_iter()
        .map(|record| record.number_block as u64)
        .collect();

        Ok(result)
    }
}
