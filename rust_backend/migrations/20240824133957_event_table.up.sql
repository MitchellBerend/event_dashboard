-- Add up migration script here
CREATE TABLE IF NOT EXISTS event (
    id SERIAL PRIMARY KEY,
    reference CHAR(64) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
