use std::sync::Arc;

use async_trait::async_trait;
use common::types::ChainConfig;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Erc721TransferData {
    pub contract_id: i32,
    pub block_number: i32,
    pub from: String,
    pub to: String,
    pub token_id: i32,
}

#[async_trait]
pub trait Erc721TransferTrait: Clone + Send + Sync + 'static {
    async fn insert_transfer(
        &self,
        transfer: Erc721TransferData,
    ) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct Erc721Repository {
    pub database_pool: Arc<PgPool>,
    pub chain_config: ChainConfig,
    pub block_id: i32,
}

impl Erc721Repository {
    pub async fn new(
        database_pool: Arc<PgPool>,
        chain_config: ChainConfig,
    ) -> Result<Self, sqlx::Error> {
        let block_id: i32 = sqlx::query_as::<_, (i32,)>(
            "SELECT id FROM block WHERE chain_id = $1 ORDER BY id DESC LIMIT 1",
        )
        .bind(chain_config.id as i32)
        .fetch_one(&*database_pool)
        .await?
        .0;

        Ok(Self {
            database_pool,
            chain_config,
            block_id,
        })
    }
}

#[async_trait]
impl Erc721TransferTrait for Erc721Repository {
    async fn insert_transfer(
        &self,
        transfer: Erc721TransferData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO erc721_transfer (contract_id, block_id, \"from\", \"to\", token_id) VALUES ($1, $2, $3, $4, $5)")
        .bind(transfer.contract_id)
        .bind(self.block_id)
        .bind(&transfer.from)
        .bind(&transfer.to)
        .bind(transfer.token_id)
        .execute(&*self.database_pool)
        .await?;

        Ok(())
    }
}
