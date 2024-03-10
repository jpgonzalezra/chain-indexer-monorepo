pub mod clients;
pub mod config;
pub mod services;

use std::sync::Arc;

use clients::{blockchain_client::BlockchainClient, redis_client::RedisClient};
use common::{database::Database, redis::redis_pool_factory};
use config::Config;
use ethers::providers::{Http, Provider};

use crate::services::sync::ChainSynchronizer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::new();
    let database = Database::new(&config.db_config)
        .await
        .expect("Database connection error.");

    let redis_config = config.clone().redis_config;
    let redis_pool = redis_pool_factory(
        redis_config.host,
        redis_config.port,
        redis_config.password,
        redis_config.db,
    )
    .await
    .expect("Error on acquiring redis connection.");

    let http_provider = Provider::<Http>::try_from(&config.rpc)
        .expect("Error on provider http creation.");

    let synchronizer = ChainSynchronizer::new(
        BlockchainClient {
            provider: Arc::new(http_provider),
        },
        RedisClient {
            pool: Arc::new(redis_pool),
        },
        config,
    );

    let mut start_block = synchronizer.start_block();
    loop {
        let end_block = synchronizer.end_block().await?;

        println!("Indexing from block {} to block {}", start_block, end_block);

        synchronizer.sync(start_block, end_block).await;

        start_block = end_block + 1;
        println!("Updating start block {}", start_block);
    }
}
