use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Erc721TransferData {
    pub contract_id: i32,
    pub block_number: i32,
    pub chain_id: i32,
    pub tx_hash: String,
    pub tx_index: u64,
    pub from: String,
    pub to: String,
    pub token_id: String,
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
}

impl Erc721Repository {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl Erc721TransferTrait for Erc721Repository {
    async fn insert_transfer(
        &self,
        transfer: Erc721TransferData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO erc721_transfer (contract_id, block_number, chain_id, tx_hash, tx_index, \"from\", \"to\", token_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(transfer.contract_id)
        .bind(transfer.block_number)
        .bind(transfer.chain_id)
        .bind(transfer.tx_hash)
        .bind(transfer.tx_index as i64)
        .bind(&transfer.from)
        .bind(&transfer.to)
        .bind(transfer.token_id)
        .execute(&*self.database_pool)
        .await?;

        Ok(())
    }
}
