-- Add migration script here
ALTER TABLE Block
    ALTER COLUMN block_number TYPE VARCHAR(12)
    USING block_number::TEXT;