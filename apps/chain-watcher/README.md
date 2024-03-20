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
TODO

### How It Works
Chain Watcher operates by directly connecting to an EVM-compatible blockchain node through its JSON-RPC interface. Once connected, it listens for new blocks and transactions, capturing this data for indexing and analysis. For every transaction processed, Chain Watcher extracts the logs and broadcasts them to a specified Redis channel. This allows any subscribed consumer services to immediately receive updates about blockchain events, opening up a wide range of possibilities for real-time data analysis, alerting, and decentralized application integration.

### Configuration Options
TODO

