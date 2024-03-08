pub mod config;
pub mod core;

use config::Config;
use ethers::providers::Middleware;

use crate::core::sync::ChainSynchronizer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let synchronizer = ChainSynchronizer::new(config.clone()).await;

    loop {
        let mut start_block = config.start_block.unwrap_or(0);

        let end_block = config
            .end_block
            .unwrap_or(synchronizer.provider.get_block_number().await?.as_u64());

        println!("Indexing from block {} to block {}", start_block, end_block);

        synchronizer.sync(start_block, end_block).await;

        start_block = end_block + 1;
        println!(
            "Updating start and end block, from {} to {}",
            start_block, end_block
        );
    }
}
