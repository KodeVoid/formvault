

CREATE TABLE users(
    user_id UUID PRIMARY KEY,
    email TEXT,  -- Encrypted strings can be variable length
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);