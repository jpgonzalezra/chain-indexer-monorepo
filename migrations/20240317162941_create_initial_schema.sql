-- Add migration script here
CREATE TABLE contract (
    id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL UNIQUE,
    chain_id INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE block (
    id SERIAL PRIMARY KEY,
    chain_id INTEGER NOT NULL,
    block_number BIGINT NOT NULL UNIQUE,
    hash VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE erc721_transfer (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL,
    block_id INTEGER NOT NULL,
    "from" VARCHAR(255) NOT NULL,
    "to" VARCHAR(255) NOT NULL,
    token_id VARCHAR NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contract(id),
    FOREIGN KEY (block_id) REFERENCES block(id)
);

CREATE TABLE erc1155_transfer (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL,
    block_id INTEGER NOT NULL,
    amounts INTEGER[] NOT NULL,
    token_ids VARCHAR[] NOT NULL,
    "from" VARCHAR(255) NOT NULL,
    "to" VARCHAR(255) NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contract(id),
    FOREIGN KEY (block_id) REFERENCES block(id)
);