-- PostgreSQL enum for role
CREATE TYPE staff_role AS ENUM (
    'admin',
    'support',
    'reviewer',
    'manager',
    'custom'
);

-- Create staff table
CREATE TABLE staff (
    staff_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    role staff_role NOT NULL,
    permissions TEXT[]
);
