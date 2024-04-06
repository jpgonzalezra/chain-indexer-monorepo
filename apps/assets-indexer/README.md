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

| Option                | Type           | Default       | Description                                                                                    | Usage                          |
| --------------------- | -------------- | ------------- | ---------------------------------------------------------------------------------------------- | ------------------------------ |
| `indexer_name`        | String         |               | Name of the indexer client. Used for logging and monitoring.                                   | `--indexer-name <NAME>`        |
| `chain_id`            | usize          | `1` (mainnet) | The chain ID number for the blockchain to synchronize with.                                    | `--chain-id <ID>`              |
| `debug`               | bool           | false         | Enables debug logging.                                                                         | `--debug`                      |
| `redis_host`          | String         | `"127.0.0.1"` | The hostname or IP address of the Redis server.                                                | `--redis-host <HOST>`          |
| `redis_port`          | u16            | `6379`        | The port number on which the Redis server is listening.                                        | `--redis-port <PORT>`          |
| `redis_password`      | Option<String> |               | The password for accessing Redis, if authentication is required. Optional.                     | `--redis-password <PASSWORD>`  |
| `redis_db`            | usize          | `1`           | The Redis database number to use.                                                              | `--redis-db <DB_NUMBER>`       |
| `db_host`             | String         | `"localhost"` | The hostname or IP address of the database server.                                             | `--db-host <HOST>`             |
| `db_port`             | u16            | `5432`        | The port number on which the database server is listening.                                     | `--db-port <PORT>`             |
| `db_username`         | String         |               | The username for accessing the database.                                                       | `--db-username <USERNAME>`     |
| `db_password`         | Option<String> |               | The password for accessing the database. Optional.                                             | `--db-password <PASSWORD>`     |
| `db_name`             | String         |               | The name of the database to use for storing indexed data.                                      | `--db-name <DATABASE_NAME>`    |

### Example Usage

```shell
assets-indexer --indexer-name "MyIndexer" --db-trans-batch-size 100 --chain-id 3 --redis-host "192.168.1.100" --redis-port 6379 --db-host "localhost" --db-port 5432 --db-username "user" --db-password "password" --db-name "indexer_db"
```
