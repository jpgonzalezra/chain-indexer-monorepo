# Monorepo for Blockchain Indexing Services

This monorepo houses two critical microservices designed for blockchain data indexing and analysis: `chain-watcher` and `assets-indexer`. Both services are tailored for EVM-compatible blockchains, providing real-time monitoring, event processing, and efficient data indexing.

## Services Overview

- **Chain Watcher**: Monitors and indexes blockchain data in real-time, broadcasting relevant transaction logs and events for downstream processing.
- **Assets indexer**: Processes blockchain events, focusing on identifying and handling ERC721 and ERC1155 assets, and stores transaction data for analysis.

## Prerequisites

Before setting up the services, ensure you have the following dependencies installed:

- Rust
- PostgreSQL
- Redis

### Rust

Rust programming language environment includes rustc (the compiler), cargo (the package manager), and standard library documentation.
Execute the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

verify

```sh
rustc --version
```

### PostgreSQL

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

### Redis

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

### Running Migrations with SQLx

To ensure the database schemas are set up correctly for both services, you need to run migrations. We use sqlx for handling migrations smoothly.

First, install sqlx-cli if you haven't already:

```sh
cargo install sqlx-cli
````

if the database does not exist

```bash
sqlx database create --database-url postgresql://{username}:${password}@{host}:{host}/{database}
```

run migrations

```bash
$ sqlx migrate run --database-url postgresql://{username}:${password}@{host}:{host}/{database}
```

## Service Configuration
Each service can be configured through command-line arguments or environment variables. See the respective service's README for detailed configuration options.

- Chain Watcher
Refer to chain-watcher/README.md for instructions on how to set up and run the Chain Watcher service.

- Assets indexer
Refer to assets-indexer/README.md for setup and usage guidelines for the Assets indexer service.