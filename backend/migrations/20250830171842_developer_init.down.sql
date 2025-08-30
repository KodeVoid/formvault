-- FormVault Database Down Migration Script
-- Version: 001_create_developers_table (DOWN)
-- Description: Rollback developers table creation

-- DROP INDEXES FIRST (in reverse order of creation)
DROP INDEX IF EXISTS idx_developers_created_at;
DROP INDEX IF EXISTS idx_developers_active;
DROP INDEX IF EXISTS idx_developers_api_key;
DROP INDEX IF EXISTS idx_developers_email;

-- DROP TABLE
DROP TABLE IF EXISTS developers;

-- REMOVE UUID EXTENSION (only if not used by other tables)
-- DROP EXTENSION IF EXISTS "uuid-ossp";-- Add down migration script here
