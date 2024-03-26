pub mod clients;
pub mod config;
pub mod services;

use crate::services::sync::ChainSynchronizer;
use clients::{blockchain_client::BlockchainClient, redis_client::RedisClient};
use common::redis::redis_pool_factory;
use config::Config;
use ethers::providers::{Http, Provider};
use services::repositories::block::{BlockRepository, BlockRepositoryTrait};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::new();

    let file_appender = rolling::never("./logs", "chain-watcher.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let debug_level = if config.debug { "debug" } else { "info" };
    let console_layer = fmt::layer().with_writer(std::io::stdout);
    let file_layer = fmt::layer().with_writer(non_blocking);
    let filter_layer = EnvFilter::new(debug_level);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(filter_layer)
        .init();

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

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_config.username,
        config.db_config.password.as_deref().unwrap_or(""),
        config.db_config.host,
        config.db_config.port,
        config.db_config.db_name
    );

    let database_pool = PgPoolOptions::new().connect(&database_url).await?;
    let block_repository =
        BlockRepository::new(Arc::new(database_pool.clone()), config.chain.clone());

    let synchronizer = ChainSynchronizer::new(
        BlockchainClient {
            provider: Arc::new(http_provider),
        },
        RedisClient {
            pool: Arc::new(redis_pool),
        },
        block_repository,
        config.clone(),
    );

    if config.reset {
        // TODO: implement
    }

    let mut start_block = synchronizer.start_block();

    let missing_blocks = synchronizer.missing_blocks().await?;
    synchronizer.sync_missing_blocks(missing_blocks).await;

    loop {
        let end_block = synchronizer.end_block().await?;

        tracing::info!("Indexing from block {} to block {}", start_block, end_block);

        synchronizer.sync(start_block, end_block).await;

        start_block = end_block + 1;
        tracing::info!("Updating start block {}", start_block);
    }
}
