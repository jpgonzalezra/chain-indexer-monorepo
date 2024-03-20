use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Erc1155TransferData {
    pub contract_id: i32,
    pub block_number: i32,
    pub chain_id: i32,
    pub tx_hash: String,
    pub tx_index: u64,
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
}

impl Erc1155Repository {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl Erc1155TransferTrait for Erc1155Repository {
    async fn insert_transfer(
        &self,
        transfer: Erc1155TransferData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO erc1155_transfer (contract_id, block_number, chain_id, tx_hash, tx_index, \"from\", \"to\", token_ids, amounts) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
         .bind(transfer.contract_id)
         .bind(transfer.block_number)
         .bind(transfer.chain_id)
         .bind(transfer.tx_hash)
         .bind(transfer.tx_index as i64)
         .bind(&transfer.from)
         .bind(&transfer.to)
         .bind(&transfer.token_ids)
         .bind(&transfer.amounts)
         .execute(&*self.database_pool)
         .await?;

        Ok(())
    }
}
