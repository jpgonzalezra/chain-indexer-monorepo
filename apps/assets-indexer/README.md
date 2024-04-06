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

Rust programming language environment includes rustc (the compiler), cargo (the package manager), and standard library documentation.
Execute the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

verify

```sh
rustc --version
```

- PostgreSQL

Execute the following command:

```sh
sudo apt install postgresql postgresql-contrib
```

verify

```sh
psql --version
```

After installation, start PostgreSQL service:

```sh
brew services start postgresql
```

- Redis

Execute the following command:

```sh
brew install redis
```

To have Redis start automatically when your machine starts:

```sh
brew services start redis
```

verify

```sh
redis-server --version
```

### Configuration Options

| Option             | Type   | Default       | Description                                                                                   | Usage                       |
| ------------------ | ------ | ------------- | --------------------------------------------------------------------------------------------- | --------------------------- |
| `indexer_name`     | String |               | Name of the indexer client. Used for logging and monitoring.                                  | `--indexer-name <NAME>`     |
| `chain_id`         | usize  | `1` (mainnet) | The chain ID number for the blockchain to synchronize with.                                   | `--chain-id <ID>`           |
| `redis_url`        | String |               | The Redis connection URL, encapsulating host, port, database number, and authentication info. | `--redis-url <REDIS_URL>`   |
| `redis_stream_key` | String |               | The key for the Redis stream where logs and data will be sent.                                | `--redis-stream-key <KEY>`  |
| `redis_group_name` | String |               | The name of the Redis group associated with the stream for distributing work among consumers. | `--redis-group-name <NAME>` |
| `db_url`           | String |               | The database connection URL, encapsulating host, port, username, password, and database name. | `--db-url <DB_URL>`         |
| `debug`            | bool   | false         | Enables debug logging. Useful for troubleshooting and development.                            | `--debug`                   |

### Example Usage

```shell
assets-indexer --indexer-name "MyIndexer" --chain-id 1 --redis-url "redis://user:password@hosts:port/db_name" --redis-stream-key "indexer-stream-key" --redis-group-name "indexer-group-name" --db-url "postgresql://user:password@host:port/db_name" --debug
```
