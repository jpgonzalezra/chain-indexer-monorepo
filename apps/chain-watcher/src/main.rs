use common::redis::redis_client_factory;
use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Transaction;
use ethers::utils::hex;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize)]
struct SummaryTransaction {
    pub hash: String,
    pub block_hash: String,
    pub block_number: u64,
    pub chain_id: String,
    pub input: String,
    pub from: String,
    pub to: String,
    pub nonce: String,
    pub transaction_index: u64,
    pub value: String,
}

impl From<Transaction> for SummaryTransaction {
    fn from(tx: Transaction) -> Self {
        SummaryTransaction {
            hash: tx.hash.to_string(),
            block_hash: tx
                .block_hash
                .map_or_else(|| "None".to_string(), |h| h.to_string()),
            block_number: tx
                .block_number
                .map_or_else(|| 0, |block_number| block_number.as_u64()),
            chain_id: tx
                .chain_id
                .map_or_else(|| 1.to_string(), |chain_id| chain_id.to_string()),
            input: hex::encode(tx.input),
            from: tx.from.to_string(),
            to: tx
                .to
                .map_or_else(|| "None".to_string(), |to| to.to_string()),
            nonce: tx.nonce.to_string(),
            transaction_index: tx
                .transaction_index
                .map_or_else(|| 0, |index| index.as_u64()),
            value: tx.value.to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut redis_conn = redis_client_factory()
        .get_async_connection()
        .await
        .expect("Error on acquiring redis connection");

    let rpc_url = env::var("ETHEREUM_RPC_URL").unwrap();
    let http_provider = Provider::<Http>::try_from(rpc_url)?;

    let provider = Arc::new(http_provider);
    let start_block = 19249009;
    let end_block = provider.get_block_number().await?;

    println!("Indexing from block {} to block {}", start_block, end_block);

    for block_number in start_block..=end_block.as_u64() {
        let block = provider.get_block_with_txs(block_number).await?;

        if let Some(block) = block {
            println!(
                "Block Number {}: {} transactions",
                block.number.unwrap(),
                block.transactions.len()
            );

            for transaction in block.transactions {
                let summary_transaction: SummaryTransaction = transaction.into();
                let message = serde_json::to_string(&summary_transaction).unwrap();
                let _: () = redis_conn
                    .xadd("ASSETS_INDEXER_STREAM", "*", &[("message", &message)])
                    .await
                    .unwrap();
            }
        }
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
