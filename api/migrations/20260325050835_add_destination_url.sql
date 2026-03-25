-- Add migration script here
ALTER TABLE affiliates ADD COLUMN destination_url TEXT NOT NULL DEFAULT 'https://example.com';