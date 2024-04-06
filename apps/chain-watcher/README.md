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

| Parameter          | Type        | Default | Description                                                                                   | Usage Example               |
| ------------------ | ----------- | ------- | --------------------------------------------------------------------------------------------- | --------------------------- |
| `reset`            | bool        | false   | If true, resets the blockchain state to restart indexing from the beginning. Optional.        | `--reset`                   |
| `debug`            | bool        | false   | Enables debug logging. Useful for troubleshooting and development.                            | `--debug`                   |
| `chain_id`         | usize       | 1       | Chain ID number to synchronize with.                                                          | `--chain-id <ID>`           |
| `rpc`              | String      |         | RPC URL to use for fetching blocks.                                                           | `--rpc <URL>`               |
| `start_block`      | Option<u64> |         | Block number to start syncing from. Optional.                                                 | `--start-block <NUMBER>`    |
| `end_block`        | Option<u64> |         | Block number to end syncing at. Optional.                                                     | `--end-block <NUMBER>`      |
| `redis_url`        | String      |         | Redis connection URL.                                                                         | `--redis-url <REDIS_URL>`   |
| `redis_stream_key` | String      |         | The key for the Redis stream where logs and data will be sent.                                | `--redis-stream-key <KEY>`  |
| `redis_group_name` | String      |         | The name of the Redis group associated with the stream for distributing work among consumers. | `--redis-group-name <NAME>` |
| `db_url`           | String      |         | Database connection URL.                                                                      | `--db-url <DB_URL>`         |

### Example Usage

```shell
$ chain_watcher --chain-id 1 --rpc "http://localhost:8545" --start-block 0 --end-block 10000 --redis-url "redis://:password@host:port/db_name" --redis-stream-key "my_stream_key" --redis-group-name "my_group_name" --db-url "postgresql://user:password@host:port/db_name"
```
