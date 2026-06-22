-- SQLx Migration: 002_add_jwt_tokens
-- Add JWT token storage to users table

ALTER TABLE users ADD COLUMN IF NOT EXISTS token TEXT DEFAULT '',
                   ADD COLUMN IF NOT EXISTS token_expiry TIMESTAMPTZ;
