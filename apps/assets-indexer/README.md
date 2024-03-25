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

| Option                | Type           | Default       | Description                                                                                    | Usage                          |
| --------------------- | -------------- | ------------- | ---------------------------------------------------------------------------------------------- | ------------------------------ |
| `indexer_name`        | String         |               | Name of the indexer client. Used for logging and monitoring.                                   | `--indexer_name <NAME>`        |
| `db_trans_batch_size` | usize          | `1`           | Determines the number of transactions to be saved in a single batch operation to the database. | `--db_trans_batch_size <SIZE>` |
| `chain_id`            | usize          | `1` (mainnet) | The chain ID number for the blockchain to synchronize with.                                    | `--chain_id <ID>`              |
| `redis_host`          | String         | `"127.0.0.1"` | The hostname or IP address of the Redis server.                                                | `--redis_host <HOST>`          |
| `redis_port`          | u16            | `6379`        | The port number on which the Redis server is listening.                                        | `--redis_port <PORT>`          |
| `redis_password`      | Option<String> |               | The password for accessing Redis, if authentication is required. Optional.                     | `--redis_password <PASSWORD>`  |
| `redis_db`            | usize          | `1`           | The Redis database number to use.                                                              | `--redis_db <DB_NUMBER>`       |
| `db_host`             | String         | `"localhost"` | The hostname or IP address of the database server.                                             | `--db_host <HOST>`             |
| `db_port`             | u16            | `5432`        | The port number on which the database server is listening.                                     | `--db_port <PORT>`             |
| `db_username`         | String         |               | The username for accessing the database.                                                       | `--db_username <USERNAME>`     |
| `db_password`         | Option<String> |               | The password for accessing the database. Optional.                                             | `--db_password <PASSWORD>`     |
| `db_name`             | String         |               | The name of the database to use for storing indexed data.                                      | `--db_name <DATABASE_NAME>`    |

### Example Usage

```shell
assets-indexer --indexer_name "MyIndexer" --db_trans_batch_size 100 --chain_id 3 --redis_host "192.168.1.100" --redis_port 6379 --db_host "localhost" --db_port 5432 --db_username "user" --db_password "password" --db_name "indexer_db"
```
