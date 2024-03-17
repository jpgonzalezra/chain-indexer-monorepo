use clap::Parser;
use common::types::{DbConfig, RedisConfig};

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

#[derive(Debug, Clone)]
pub struct Config {
    pub indexer_name: String,
    pub db_config: DbConfig,
    pub redis_config: RedisConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let args = AssetsIndexerArgs::parse();

        Self {
            indexer_name: args.indexer_name,
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
        }
    }
}
