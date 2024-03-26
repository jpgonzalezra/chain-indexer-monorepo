use std::collections::HashMap;

use clap::Parser;
use common::types::{ChainConfig, DbConfig, RedisConfig};
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
        help = "Specifies the number of transactions to be saved in a single batch operation to the database.",
        default_value_t = 1
    )]
    pub db_trans_batch_size: usize,
    #[arg(
        long,
        help = "Chain ID number to synchronize with.",
        default_value_t = 1
    )]
    pub chain_id: usize,
    #[arg(long, help = "Redis host value.", default_value = "127.0.0.1")]
    pub redis_host: String,
    #[arg(long, help = "Redis port value.", default_value = "6379")]
    pub redis_port: u16,
    #[arg(long, help = "Redis password value. [optional]")]
    pub redis_password: Option<String>,
    #[arg(long, help = "Redis db value.", default_value_t = 1)]
    pub redis_db: usize,
    #[arg(long, help = "Database host value.", default_value = "localhost")]
    pub db_host: String,
    #[arg(long, help = "Database port value.", default_value = "5432")]
    pub db_port: u16,
    #[arg(long, help = "Database username value.")]
    pub db_username: String,
    #[arg(long, help = "Database password value. [optional]")]
    pub db_password: Option<String>,
    #[arg(long, help = "Database name value.")]
    pub db_name: String,
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
    pub db_config: DbConfig,
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
            db_config: DbConfig {
                host: args.db_host,
                port: args.db_port,
                password: args.db_password,
                db_name: args.db_name,
                username: args.db_username,
                db_trans_batch_size: args.db_trans_batch_size,
            },
            redis_config: RedisConfig {
                host: args.redis_host,
                port: args.redis_port,
                password: args.redis_password,
                db: args.redis_db,
            },
            debug: args.debug,
        }
    }
}
