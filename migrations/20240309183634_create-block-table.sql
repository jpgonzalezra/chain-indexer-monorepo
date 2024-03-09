-- Add migration script here
CREATE TABLE Block (
    id SERIAL PRIMARY KEY,
    block_number BIGINT UNIQUE NOT NULL,
    hash VARCHAR(255) UNIQUE NOT NULL
);