-- Add migration script here
ALTER TABLE Block ADD COLUMN chain_id BIGINT NOT NULL;