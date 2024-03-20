-- Add migration script here
CREATE TABLE contract (
    id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL UNIQUE,
    chain_id INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE block (
    block_number BIGINT NOT NULL,
    chain_id INTEGER NOT NULL,
    hash VARCHAR(255) NOT NULL UNIQUE
    PRIMARY KEY (chain_id, block_number)
);

CREATE TABLE erc721_transfer (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL,
    chain_id INTEGER NOT NULL,
    block_number BIGINT NOT NULL,
    tx_hash VARCHAR(255) NOT NULL, 
    tx_index BIGINT NOT NULL;
    "from" VARCHAR(255) NOT NULL,
    "to" VARCHAR(255) NOT NULL,
    token_id VARCHAR NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contract(id),
    FOREIGN KEY (chain_id, block_number) REFERENCES block(chain_id, block_number)
);

CREATE TABLE erc1155_transfer (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL,
    chain_id INTEGER NOT NULL,
    block_number BIGINT NOT NULL,
    tx_hash VARCHAR(255) NOT NULL,
    tx_index BIGINT NOT NULL;
    amounts VARCHAR[] NOT NULL,
    token_ids VARCHAR[] NOT NULL,
    "from" VARCHAR(255) NOT NULL,
    "to" VARCHAR(255) NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contract(id),
    FOREIGN KEY (chain_id, block_number) REFERENCES block(chain_id, block_number)
);