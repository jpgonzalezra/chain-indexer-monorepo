use std::sync::Arc;

use async_trait::async_trait;
use common::types::ChainConfig;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Erc1155TransferData {
    pub contract_id: i32,
    pub block_number: i32,
    pub from: String,
    pub to: String,
    pub token_ids: Vec<String>,
    pub amounts: Vec<String>,
}

#[async_trait]
pub trait Erc1155TransferTrait: Clone + Send + Sync + 'static {
    async fn insert_transfer(
        &self,
        transfer: Erc1155TransferData,
    ) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct Erc1155Repository {
    pub database_pool: Arc<PgPool>,
    pub chain_config: ChainConfig,
    pub block_id: i32,
}

impl Erc1155Repository {
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
impl Erc1155TransferTrait for Erc1155Repository {
    async fn insert_transfer(
        &self,
        transfer: Erc1155TransferData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO erc1155_transfer (contract_id, block_id, \"from\", \"to\", token_ids, amounts) VALUES ($1, $2, $3, $4, $5, $6)")
         .bind(transfer.contract_id)
         .bind(self.block_id)
         .bind(&transfer.from)
         .bind(&transfer.to)
         .bind(&transfer.token_ids)
         .bind(&transfer.amounts)
         .execute(&*self.database_pool)
         .await?;

        Ok(())
    }
}
