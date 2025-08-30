-- Drop tables (reverse order due to dependencies)
DROP TABLE IF EXISTS form_submissions CASCADE;
DROP TABLE IF EXISTS field_definitions CASCADE;
DROP TABLE IF EXISTS form_schemas CASCADE;

-- Drop enums
DROP TYPE IF EXISTS form_field_type CASCADE;
DROP TYPE IF EXISTS form_type CASCADE;
