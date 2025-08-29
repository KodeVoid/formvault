-- Add migration script here
-- Create FormType Enum


CREATE TYPE formtype as ENUM(
	'Newsletter',
	'Contact',
	'Survey',
	'Registration',
	
);