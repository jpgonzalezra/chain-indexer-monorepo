use std::sync::Arc;

use async_trait::async_trait;
use common::types::ChainConfig;
use sqlx::{FromRow, PgPool};

pub enum Bind {
    BIGINT(i64),
    INT(i32),
}

#[derive(Debug)]
pub struct Block {
    pub block_number: u64,
    pub chain_id: u32,
}

#[derive(Debug, FromRow)]
pub struct BlockNumber {
    block_number: i64,
}

#[async_trait]
pub trait BlockRepositoryTrait: Clone + Send + Sync + 'static {
    fn new(database_pool: Arc<PgPool>, chain_config: ChainConfig) -> Self;
    async fn get_indexed_blocks(&self) -> Result<Vec<u64>, sqlx::Error>;
    async fn insert_block(&self, blocks: Block) -> Result<(), sqlx::Error>;
    async fn insert_blocks_bulk(&self, blocks: &[Block]) -> Result<(), sqlx::Error>;
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
            "SELECT block_number FROM block WHERE chain_id = $1",
        )
        .bind(self.chain_config.id as i32)
        .fetch_all(&*pool)
        .await?
        .into_iter()
        .map(|record| record.block_number as u64)
        .collect();

        Ok(result)
    }

    async fn insert_block(&self, block: Block) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO block (block_number, chain_id) VALUES ($1, $2)";

        sqlx::query(query)
            .bind(block.block_number as i64)
            .bind(block.chain_id as i32)
            .execute(&*self.database_pool)
            .await?;

        Ok(())
    }

    async fn insert_blocks_bulk(&self, blocks: &[Block]) -> Result<(), sqlx::Error> {
        if blocks.is_empty() {
            return Ok(());
        }

        let mut query =
            String::from("INSERT INTO block (block_number, chain_id) VALUES ");

        let mut binds: Vec<Bind> = vec![];
        for (index, block) in blocks.iter().enumerate() {
            if index > 0 {
                query.push_str(", ");
            }
            let placeholder_index = index * 2 + 1;
            query.push_str(&format!(
                "(${}, ${})",
                placeholder_index,
                placeholder_index + 1,
            ));
            binds.push(Bind::BIGINT(block.block_number as i64));
            binds.push(Bind::INT(block.chain_id as i32));
        }

        let mut query_builder = sqlx::query(&query);

        for bind in binds.iter() {
            match bind {
                Bind::BIGINT(i64_data) => query_builder = query_builder.bind(i64_data),
                Bind::INT(i32_data) => query_builder = query_builder.bind(i32_data),
            }
        }

        query_builder.execute(&*self.database_pool).await?;

        Ok(())
    }
}
