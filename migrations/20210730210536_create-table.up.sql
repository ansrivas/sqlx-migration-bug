-- Add up migration script here
CREATE TABLE IF NOT EXISTS test (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    modified_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    name TEXT
);