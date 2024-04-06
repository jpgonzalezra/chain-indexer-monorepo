use std::collections::HashMap;

use clap::Parser;
use common::types::{ChainConfig, RedisConfig};
use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
#[command(
    name = "Chain Watcher",
    about = "Scalable Chain Watcher for EVM compatible blockchains."
)]
pub struct AssetsIndexerArgs {
    #[arg(long, help = "Indexer client name.")]
    pub indexer_name: String,
    #[arg(
        long,
        help = "Chain ID number to synchronize with.",
        default_value_t = 1
    )]
    pub chain_id: usize,
    #[arg(long, help = "Redis connection URL.")]
    pub redis_url: String,
    #[arg(long, help = "Redis stream key")]
    pub redis_stream_key: String,
    #[arg(long, help = "Redis group name.")]
    pub redis_group_name: String,
    #[arg(long, help = "Database connection URL.")]
    pub db_url: String,
    #[arg(
        long,
        help = "Enables debug logging. Useful for troubleshooting and development. [optional]",
        default_value_t = false
    )]
    pub debug: bool,
}

static CHAIN_CONFIGS: Lazy<HashMap<usize, ChainConfig>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        1,
        ChainConfig {
            id: 1,
            name: "Ethereum".to_string(),
        },
    );
    m
});

#[derive(Debug, Clone)]
pub struct Config {
    pub indexer_name: String,
    pub chain: ChainConfig,
    pub db_url: String,
    pub redis_config: RedisConfig,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let args = AssetsIndexerArgs::parse();
        let chain = CHAIN_CONFIGS
            .get(&args.chain_id)
            .expect("Default chain error.")
            .clone();
        Self {
            indexer_name: args.indexer_name,
            chain,
            db_url: args.db_url,
            redis_config: RedisConfig {
                url: args.redis_url,
                stream_key: args.redis_stream_key,
                group_name: args.redis_group_name,
            },
            debug: args.debug,
        }
    }
}
