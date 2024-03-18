use clap::Parser;
use common::types::{ChainConfig, DbConfig, RedisConfig};
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
        help = "Number of transactions to fetch in parallel.",
        default_value_t = 8
    )]
    pub tx_batch_size: usize,
    #[arg(
        long,
        help = "Specifies the number of transactions to be saved in a single batch operation to the database. [optional]",
        default_value_t = 3
    )]
    pub db_trans_batch_size: usize,
    #[arg(
        long,
        help = "Enables resetting the blockchain state to restart indexing from the beginning. Use this flag to clear existing data and re-initialize the index. [optional]",
        default_value_t = false
    )]
    pub reset: bool,
    #[arg(
        long,
        help = "Chain ID number to synchronize with.",
        default_value_t = 1
    )]
    pub chain_id: usize,
    #[arg(long, help = "Enables debug logging.", default_value_t = false)]
    pub debug: bool,
    #[arg(long, help = "RPC URL to use for fetching blocks.")]
    pub rpc: String,
    #[arg(long, help = "Block number to start syncing from. [optional]")]
    pub start_block: Option<u64>,
    #[arg(long, help = "Block number to end syncing at. [optional]")]
    pub end_block: Option<u64>,
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
    pub tx_batch_size: usize,
    pub chain: ChainConfig,
    pub db_config: DbConfig,
    pub redis_config: RedisConfig,
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub rpc: String,
    pub num_workers: usize,
    pub reset: bool,
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
            tx_batch_size: args.tx_batch_size,
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
            start_block: args.start_block,
            end_block: args.end_block,
            rpc,
            num_workers: num_cpus::get(),
            reset: args.reset,
        }
    }
}
