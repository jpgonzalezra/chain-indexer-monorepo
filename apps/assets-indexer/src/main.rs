pub mod config;
pub mod services;

use std::sync::Arc;

use common::{redis::redis_client_factory, types::SummaryLog};
use config::Config;
use redis::{
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands, RedisResult, Value,
};
use services::{
    processors::{
        erc1155_transfer_batch_event_processor::Erc1155TransferBatchProcessor,
        erc1155_transfer_single_event_processor::Erc1155TransferSingleProcessor,
        erc721_transfer_event_processor::Erc721TransferProcessor,
        event_processor::{EventProcessorRequest, EventProcessorService},
    },
    repositories::{
        contract_repository::ContractRepository, erc1155_repository::Erc1155Repository,
        erc721_repository::Erc721Repository,
    },
};
use sqlx::postgres::PgPoolOptions;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

async fn ensure_stream_and_group_exist(
    conn: &mut redis::aio::Connection,
    stream_key: &str,
    group_name: &str,
) -> RedisResult<()> {
    // Try to create the group. If the group already exists, this command will fail, so ignore the specific error of an existing group.
    let create_group_result: RedisResult<()> = conn
        .xgroup_create_mkstream(stream_key, group_name, "$")
        .await;
    match create_group_result {
        Ok(_) => tracing::info!(
            "Group '{}' created for the stream '{}'.",
            group_name,
            stream_key
        ),
        Err(ref e)
            if e.kind() == redis::ErrorKind::ExtensionError
                && e.to_string().contains("BUSYGROUP") =>
        {
            tracing::info!(
                "The group '{}' already exists for the stream '{}'.",
                group_name,
                stream_key
            );
        }
        Err(e) => return Err(e), // Propagate other errors.
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let file_appender = rolling::daily("./logs", "assets-indexer.log");
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

    let redis_config = config.redis_config;

    let mut redis_conn = redis_client_factory(redis_config.url)
        .expect("Error on acquiring redis client")
        .get_async_connection()
        .await
        .expect("Error on acquiring redis connection");

    let database_pool = PgPoolOptions::new().connect(&config.db_url).await?;

    let mut processor = EventProcessorService::new();
    processor.add_processor(Box::new(Erc721TransferProcessor {
        erc721_repository: Erc721Repository::new(Arc::new(database_pool.clone())),
        contract_repository: ContractRepository::new(Arc::new(database_pool.clone())),
    }));
    processor.add_processor(Box::new(Erc1155TransferSingleProcessor {
        erc1155_repository: Erc1155Repository::new(Arc::new(database_pool.clone())),
        contract_repository: ContractRepository::new(Arc::new(database_pool.clone())),
    }));
    processor.add_processor(Box::new(Erc1155TransferBatchProcessor {
        erc1155_repository: Erc1155Repository::new(Arc::new(database_pool.clone())),
        contract_repository: ContractRepository::new(Arc::new(database_pool.clone())),
    }));

    let stream_key = redis_config.stream_key;
    let group = redis_config.group_name;
    ensure_stream_and_group_exist(&mut redis_conn, &stream_key, &group).await?;

    let consumer_name: String = config.indexer_name;
    let opts: StreamReadOptions =
        StreamReadOptions::default().group(&group, consumer_name);

    loop {
        let results: RedisResult<StreamReadReply> = redis_conn
            .xread_options(&[&stream_key], &[">"], &opts)
            .await;

        match results {
            Ok(reply) => {
                for stream in reply.keys {
                    for message in stream.ids {
                        if let Some(Value::Data(bytes)) = message.map.get("message") {
                            let json_data =
                                String::from_utf8(bytes.clone()).unwrap_or_default();

                            let logs: Vec<SummaryLog> = serde_json::from_str(&json_data)?;
                            for log in logs {
                                let address = log.address;
                                let data = log.data;
                                let topics = log.topics;
                                processor
                                    .process_and_store_if_apply(&EventProcessorRequest {
                                        tx_hash: log.transaction_hash.unwrap_or_default(),
                                        tx_index: log
                                            .transaction_index
                                            .unwrap_or_default(),
                                        address,
                                        data,
                                        topic0: topics
                                            .first()
                                            .cloned()
                                            .unwrap_or_default(),
                                        block_number: log.block_number,
                                        chain_id: config.chain.clone().id,
                                        topic1: topics.get(1).cloned(),
                                        topic2: topics.get(2).cloned(),
                                        topic3: topics.get(3).cloned(),
                                    })
                                    .await;
                            }
                        }
                    }
                }
            }
            Err(e) => tracing::error!("Error reading from stream: {}", e),
        }
    }
}
