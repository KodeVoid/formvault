-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    email VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW()
);