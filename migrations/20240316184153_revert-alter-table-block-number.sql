-- Add migration script here
ALTER TABLE Block ALTER COLUMN block_number TYPE BIGINT USING block_number::BIGINT;