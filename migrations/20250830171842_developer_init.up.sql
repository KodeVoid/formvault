-- FormVault Database Migration Script
-- Version: 001_create_developers_table
-- Description: Create developers table for FormVault SaaS

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE DEVELOPERS TABLE
CREATE TABLE developers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    public_key TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    api_key TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- CREATE INDEXES FOR PERFORMANCE
CREATE INDEX idx_developers_email ON developers(email);
CREATE INDEX idx_developers_api_key ON developers(api_key);
CREATE INDEX idx_developers_active ON developers(is_active);
CREATE INDEX idx_developers_created_at ON developers(created_at);

-- ADD COMMENTS FOR DOCUMENTATION
COMMENT ON TABLE developers IS 'Stores developer accounts for FormVault SaaS';
COMMENT ON COLUMN developers.id IS 'Unique identifier for developer';
COMMENT ON COLUMN developers.name IS 'Developer full name';
COMMENT ON COLUMN developers.email IS 'Developer email address (unique)';
COMMENT ON COLUMN developers.public_key IS 'RSA public key for encrypting form data';
COMMENT ON COLUMN developers.created_at IS 'Account creation timestamp';
COMMENT ON COLUMN developers.api_key IS 'API key for authentication (unique)';
COMMENT ON COLUMN developers.is_active IS 'Account status flag';