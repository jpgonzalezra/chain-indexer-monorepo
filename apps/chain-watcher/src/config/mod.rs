use clap::Parser;
use common::types::{ChainConfig, RedisConfig};
use hashbrown::HashMap;
use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
#[command(
    name = "Chain Watcher",
    about = "Scalable Chain Watcher for EVM compatible blockchains."
)]
pub struct ChainWatcherArgs {
    #[arg(
        long,
        help = "Enables resetting the blockchain state to restart indexing from the beginning. Use this flag to clear existing data and re-initialize the index. [optional]",
        default_value_t = false
    )]
    pub reset: bool,
    #[arg(
        long,
        help = "Enables debug logging. Useful for troubleshooting and development. [optional]",
        default_value_t = false
    )]
    pub debug: bool,
    #[arg(
        long,
        help = "Chain ID number to synchronize with.",
        default_value_t = 1
    )]
    pub chain_id: usize,
    #[arg(long, help = "RPC URL to use for fetching blocks.")]
    pub rpc: String,
    #[arg(long, help = "Block number to start syncing from. [optional]")]
    pub start_block: Option<u64>,
    #[arg(long, help = "Block number to end syncing at. [optional]")]
    pub end_block: Option<u64>,
    #[arg(long, help = "Redis connection URL.")]
    pub redis_url: String,
    #[arg(long, help = "Redis stream key")]
    pub redis_stream_key: String,
    #[arg(long, help = "Redis group name.")]
    pub redis_group_name: String,
    #[arg(long, help = "Database connection URL.")]
    pub db_url: String,
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
    pub chain: ChainConfig,
    pub db_url: String,
    pub redis_config: RedisConfig,
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub rpc: String,
    pub num_workers: usize,
    pub reset: bool,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let args = ChainWatcherArgs::parse();

        let chain = CHAIN_CONFIGS
            .get(&args.chain_id)
            .expect("Default chain error.")
            .clone();
        let rpc: String = args.rpc;

        Self {
            chain,
            db_url: args.db_url,
            redis_config: RedisConfig {
                url: args.redis_url,
                stream_key: args.redis_stream_key,
                group_name: args.redis_group_name,
            },
            start_block: args.start_block,
            end_block: args.end_block,
            rpc,
            num_workers: num_cpus::get(),
            reset: args.reset,
            debug: args.debug,
        }
    }
}
