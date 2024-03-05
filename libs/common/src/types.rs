use ethers::{
    types::{Log, Transaction},
    utils::hex,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SummaryLog {
    pub address: String,
    pub data: String,
    pub block_number: u64,
    pub transaction_hash: Option<String>,
    pub transaction_index: Option<u64>,
    pub topics: Vec<String>,
    pub log_index: Option<String>,
}

impl From<Log> for SummaryLog {
    fn from(log: Log) -> Self {
        SummaryLog {
            address: log.address.to_string(),
            data: log.data.to_string(),
            block_number: log
                .block_number
                .map_or_else(|| 0, |block_number| block_number.as_u64()),
            transaction_hash: log
                .transaction_hash
                .map_or_else(|| None, |h| Some(h.to_string())),
            transaction_index: log
                .transaction_index
                .map_or(None, |index| Some(index.as_u64())),
            topics: log
                .topics
                .into_iter()
                .map(|topic| topic.to_string())
                .collect(),
            log_index: log.log_index.map_or(None, |index| Some(index.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SummaryTransaction {
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