pub mod config;

use common::redis::redis_client_factory;
use config::Config;
use redis::{
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands, RedisResult,
};
use std::time::Duration;
use tokio::time::sleep;

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
        Ok(_) => println!(
            "Group '{}' created for the stream '{}'.",
            group_name, stream_key
        ),
        Err(ref e)
            if e.kind() == redis::ErrorKind::ExtensionError
                && e.to_string().contains("BUSYGROUP") =>
        {
            println!(
                "The group '{}' already exists for the stream '{}'.",
                group_name, stream_key
            );
        }
        Err(e) => return Err(e), // Propagate other errors.
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let redis_config = config.redis_config;

    let mut redis_conn = redis_client_factory(
        redis_config.host,
        redis_config.port,
        redis_config.password,
        redis_config.db,
    )
    .expect("Error on acquiring redis client")
    .get_async_connection()
    .await
    .expect("Error on acquiring redis connection");

    ensure_stream_and_group_exist(
        &mut redis_conn,
        "ASSETS_INDEXER_STREAM",
        "ASSETS_INDEXER_GROUP",
    )
    .await?;

    let consumer_name: String = config.indexer_name;
    let opts: StreamReadOptions =
        StreamReadOptions::default().group("ASSETS_INDEXER_GROUP", consumer_name);

    let stream_key = "ASSETS_INDEXER_STREAM";

    loop {
        let results: RedisResult<StreamReadReply> =
            redis_conn.xread_options(&[stream_key], &[">"], &opts).await;

        match results {
            Ok(reply) => {
                for stream in reply.keys {
                    for message in stream.ids {
                        println!("ID: {}, Values: {:?}", message.id, message.map);
                        println!("-----");
                    }
                }
            }
            Err(e) => println!("Error reading from stream: {}", e),
        }

        sleep(Duration::from_millis(10)).await;
    }
}
