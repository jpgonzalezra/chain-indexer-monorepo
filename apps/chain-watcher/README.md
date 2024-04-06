# Chain Watcher

Chain Watcher is a scalable and efficient tool specifically designed for EVM-compatible blockchains. It facilitates historical and real-time monitoring and indexing of blockchain data, offering a robust solution for projects that require up-to-the-minute data from the blockchain. By seamlessly interacting with blockchain nodes via JSON-RPC, Chain Watcher captures block and transaction data, indexing essential information into a PostgreSQL database.

Beyond simple data indexing, Chain Watcher's standout feature is its ability to broadcast transaction logs and other significant events to a dedicated Redis channel. This functionality enables data streaming, allowing downstream consumers to subscribe to these Redis channels and react or process blockchain events as they occur.

### Key Features

- Real-time Blockchain Monitoring: Continuously fetches new blocks and transactions from specified EVM-compatible blockchains, ensuring timely data capture.
- Efficient Data Indexing: Stores critical blocks and transaction details into PostgreSQL.
- Data Broadcasting: Sends transaction logs and blockchain events to a Redis channel, enabling real-time data consumption and processing by downstream applications or services.
- Resilience and Scalability: Utilizes Redis for efficient data caching and state management, supporting robust operation and scalability even during high-volume periods.
- Flexible and Configurable: Easily configurable to target various blockchains, adjust operational parameters, and cater to specific monitoring and data processing needs.

### Getting Started

To run chain-watcher, you need to have installed:

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

### How It Works

Chain Watcher operates by directly connecting to an EVM-compatible blockchain node through its JSON-RPC interface. Once connected, it listens for new blocks and transactions, capturing this data for indexing and analysis. For every transaction processed, Chain Watcher extracts the logs and broadcasts them to a specified Redis channel. This allows any subscribed consumer services to immediately receive updates about blockchain events, opening up a wide range of possibilities for real-time data analysis, alerting, and decentralized application integration.

### Configuration Options

| Parameter        | Type           | Default   | Description                                                                            | Usage Example                 |
| ---------------- | -------------- | --------- | -------------------------------------------------------------------------------------- | ----------------------------- |
| `tx_batch_size`  | usize          | 8         | Number of transactions to fetch in parallel.                                           | `--tx-batch-size <SIZE>`      |
| `reset`          | bool           | false     | If true, resets the blockchain state to restart indexing from the beginning. Optional. | `--reset`                     |
| `chain_id`       | usize          | 1         | Chain ID number to synchronize with.                                                   | `--chain-id <ID>`             |
| `debug`          | bool           | false     | Enables debug logging.                                                                 | `--debug`                     |
| `rpc`            | String         |           | RPC URL to use for fetching blocks.                                                    | `--rpc <URL>`                 |
| `start_block`    | Option<u64>    |           | Block number to start syncing from. Optional.                                          | `--start-block <NUMBER>`      |
| `end_block`      | Option<u64>    |           | Block number to end syncing at. Optional.                                              | `--end-block <NUMBER>`        |
| `redis_host`     | String         | 127.0.0.1 | Redis host value.                                                                      | `--redis-host <HOST>`         |
| `redis_port`     | u16            | 6379      | Redis port value.                                                                      | `--redis-port <PORT>`         |
| `redis_password` | Option<String> |           | Redis password value. Optional.                                                        | `--redis-password <PASSWORD>` |
| `redis_db`       | usize          | 1         | Redis db value.                                                                        | `--redis-db <DB_NUMBER>`      |
| `db_host`        | String         | localhost | Database host value.                                                                   | `--db-host <HOST>`            |
| `db_port`        | u16            | 5432      | Database port value.                                                                   | `--db-port <PORT>`            |
| `db_username`    | String         |           | Database username value.                                                               | `--db-username <USERNAME>`    |
| `db_password`    | Option<String> |           | Database password value. Optional.                                                     | `--db-password <PASSWORD>`    |
| `db_name`        | String         |           | Database name value.                                                                   | `--db-name <DATABASE_NAME>`   |

### Example Usage

```shell
chain_watcher --tx-batch-size 10 --db-trans-batch-size 5 --reset --chain-id 1 --debug --rpc "http://localhost:8545" --start-block 0 --redis-host "127.0.0.1" --redis-port 6379 --db-host "localhost" --db-port 5432 --db-username "user" --db-password "pass" --db-name "indexer_db"
```
