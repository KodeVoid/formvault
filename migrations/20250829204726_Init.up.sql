-- ========================
-- ENUMS
-- ========================

-- form_type enum
CREATE TYPE form_type AS ENUM (
    'newsletter',
    'contact',
    'survey',
    'registration',
    'custom'
);

-- field_type enum
CREATE TYPE form_field_type AS ENUM (
    'text',
    'email',
    'phone',
    'number',
    'date',
    'select',
    'checkbox',
    'file'
);

-- ========================
-- TABLES
-- ========================

-- forms table (FormSchema)
CREATE TABLE forms (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    developer_id UUID NOT NULL,
    public_key TEXT NOT NULL,
    webhook_url TEXT,
    form_type form_type NOT NULL DEFAULT 'custom',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- field_definitions table (FieldDefinition)
CREATE TABLE field_definitions (
    id UUID PRIMARY KEY,
    form_id UUID NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    field_type form_field_type NOT NULL,
    required BOOLEAN NOT NULL DEFAULT false,
    validation_regex TEXT,
    custom_error_message TEXT
);

-- submissions table (FormSubmission)
CREATE TABLE submissions (
    id UUID PRIMARY KEY,
    form_schema_id UUID NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    encrypted_data TEXT NOT NULL,
    encrypted_key TEXT NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    status TEXT NOT NULL
        CHECK (status IN ('New', 'Processing', 'Delivered', 'Archived')
               OR status LIKE 'Failed%')
);
