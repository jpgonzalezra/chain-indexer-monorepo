# Indexer Assets

Indexer Assets is a tool designed to receive blockchain events (chain watcher), process these events according to different strategies (e.g., ERC721 and ERC1155 transfers), and save relevant transactions in a database for further analysis or query.

### Key Features

- Real-time blockchain event processing.
- Support for ERC721 and ERC1155 event handling.
- Transaction data storage in PostgreSQL database.
- Flexible configuration via configuration.
- Integration with Redis for message queue management.

### Getting Started

To run indexer-assets, you need to have installed:

- Rust
- PostgreSQL
- Redis

### Configuration Options

- indexer_name (String): Name of the indexer client. This identifier is used for logging and monitoring.
```
Usage: --indexer_name <NAME>
```
- db_trans_batch_size (usize): Determines the number of transactions that are saved in a single batch operation to the database. This can affect performance and throughput.
```
Default: 1
Usage: --db_trans_batch_size <SIZE>
```
- chain_id (usize): The chain ID number for the blockchain you wish to synchronize with. This ensures that the indexer is connected to the correct network.
```
Default: 1 (mainnet)
Usage: --chain_id <ID>
```
- redis_host (String): The hostname or IP address of the Redis server. Redis is used for caching and coordination between instances.
```
Default: "127.0.0.1"
Usage: --redis_host <HOST>
```
- redis_port (u16): The port number on which the Redis server is listening.
```
Default: 6379
Usage: --redis_port <PORT>
```
- redis_password (Option<String>): The password for accessing Redis, if authentication is required. This field is optional.
```
Usage: --redis_password <PASSWORD>
```
- redis_db (usize): The Redis database number to use.
```
Default: 1
Usage: --redis_db <DB_NUMBER>
```
- db_host (String): The hostname or IP address of the database server. The database is used for persisting indexed data.
```
Default: "localhost"
Usage: --db_host <HOST>
```
- db_port (u16): The port number on which the database server is listening.
```
Default: 5432
Usage: --db_port <PORT>
```
- db_username (String): The username for accessing the database.
```
Usage: --db_username <USERNAME>
```
- db_password (Option<String>): The password for accessing the database. This field is optional.
```
Usage: --db_password <PASSWORD>
```
- db_name (String): The name of the database to use for storing indexed data.
```
Usage: --db_name <DATABASE_NAME>
```