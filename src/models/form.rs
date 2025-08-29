use crate::repositories::form::save_submission;
use crate::repositories::form::update_submission_status;
use crate::webhook::send_webhook;
use crate::repositories::encryption::encrypt_form_data;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::errors::FormVaultError;

// Base trait for all forms
trait Form {
    fn form_id(&self) -> Uuid;
    fn form_type(&self) -> FormType;
    fn created_at(&self) -> DateTime<Utc>;
    fn delete(self) -> Result<(), FormVaultError>;
    
    fn is_form() -> bool {
        true
    }
}

// Instead of separate structs for each form type, use composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSchema {
    pub id: Uuid,
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub developer_id: Uuid,
    pub public_key: String, // Developer's public key for encryption
    pub created_at: DateTime<Utc>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub id: String,
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub validation_rules: ValidationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text { max_length: Option<usize> },
    Email,
    Phone { country_code: Option<String> },
    Number { min: Option<f64>, max: Option<f64> },
    Date { format: String },
    Select { options: Vec<String> },
    Checkbox,
    File { allowed_types: Vec<String>, max_size_mb: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub regex: Option<String>,
    pub custom_error_message: Option<String>,
}

// This is what gets stored for each submission
#[derive(Debug, Serialize, Deserialize)]
pub struct FormSubmission {
    pub id: Uuid,                      // Unique ID for THIS submission
    pub form_schema_id: Uuid,          // Links to the form template
    pub encrypted_data: String,        // The actual form data, encrypted
    pub encrypted_key: String,         // AES key encrypted with dev's public key
    pub metadata: SubmissionMetadata,  // Non-sensitive data
    pub created_at: DateTime<Utc>,
    pub status: SubmissionStatus,      // New, processed, failed, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubmissionStatus {
    New,
    Processing,
    Delivered,       // Successfully sent to webhook
    Failed(String),  // Failed with error message
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub country: Option<String>,
    // No PII here - only for analytics
}

// Enum for different form types (optional, for categorization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormType {
    Newsletter,
    Contact,
    Survey,
    Registration,
    Custom(String),
}


// Implementation
impl FormSchema {
    /// Get all submissions for this form
    /// TODO: Implement database query logic
    pub async fn get_submissions(&self, limit: Option<usize>) -> Result<Vec<FormSubmission>, FormVaultError> {
        unimplemented!("Database query for submissions")
    }

    /// Get submissions with pagination
    /// TODO: Implement paginated database query
    pub async fn get_submissions_paginated(
        &self, 
        page: usize, 
        per_page: usize
    ) -> Result<(Vec<FormSubmission>, usize), FormVaultError> {
        unimplemented!("Paginated query")
    }

    /// Get submissions within date range
    /// TODO: Implement date range database query
    pub async fn get_submissions_by_date(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<FormSubmission>, FormVaultError> {
        unimplemented!("Date range query")
    }

    /// Get submission count for analytics
    /// TODO: Implement count query
    pub async fn get_submission_count(&self) -> Result<usize, FormVaultError> {
        unimplemented!("Count query")
    }

    /// Process a new submission
    pub async fn process_submission(
        &self,
        raw_data: HashMap<String, String>,
        metadata: SubmissionMetadata,
    ) -> Result<FormSubmission, FormVaultError> {
        // 1. Validate the submission
        self.validate_submission(&raw_data)?;
        
        // 2. Encrypt the data
        let (encrypted_data, encrypted_key) = encrypt_form_data(
            &raw_data, 
            &self.public_key
        ).await?;
        
        // 3. Create submission record
        let submission = FormSubmission {
            id: Uuid::new_v4(),
            form_schema_id: self.id,
            encrypted_data,
            encrypted_key,
            metadata,
            created_at: Utc::now(),
            status: SubmissionStatus::New,
        };
        
        // 4. Save to database
        save_submission(&submission).await?;
        
        // 5. Send to webhook (if configured)
        if let Some(webhook_url) = &self.webhook_url {
            send_webhook(&submission, webhook_url).await?;
        }
        
        Ok(submission)
    }

    pub fn new(
        name: String,
        developer_id: Uuid,
        public_key: String,
        fields: Vec<FieldDefinition>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            fields,
            developer_id,
            public_key,
            created_at: Utc::now(),
            webhook_url: None,
        }
    }

    pub fn add_field(&mut self, field: FieldDefinition) {
        self.fields.push(field);
    }

    pub fn validate_submission(&self, data: &HashMap<String, String>) -> Result<(), FormVaultError> {
        let mut errors = Vec::new();

        for field in &self.fields {
            let value = data.get(&field.id);
            
            // Check required fields
            if field.required && (value.is_none() || value.unwrap().is_empty()) {
                errors.push(format!("Field '{}' is required", field.name));
                continue;
            }

            // Type-specific validation
            if let Some(val) = value {
                match &field.field_type {
                    FieldType::Email => {
                        // TODO: Consider using a proper email regex for robust validation
                        if !val.contains('@') {
                            errors.push(format!("Invalid email in field '{}'", field.name));
                        }
                    },
                    FieldType::Text { max_length } => {
                        if let Some(max) = max_length {
                            if val.len() > *max {
                                errors.push(format!("Field '{}' exceeds maximum length", field.name));
                            }
                        }
                    },
                    FieldType::Number { min, max } => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                if let Some(min_val) = min {
                                    if num < *min_val {
                                        errors.push(format!("Number too small in field '{}'", field.name));
                                    }
                                }
                                if let Some(max_val) = max {
                                    if num > *max_val {
                                        errors.push(format!("Number too large in field '{}'", field.name));
                                    }
                                }
                            },
                            Err(_) => errors.push(format!("Invalid number in field '{}'", field.name)),
                        }
                    },
                    // TODO: Implement validation for other FieldType variants
                    _ => {}
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(FormVaultError::ValidationFailed(errors))
        }
    }

    pub fn newsletter_form(developer_id: Uuid, public_key: String) -> Self {
        let fields = vec![
            FieldDefinition {
                id: "email".to_string(),
                name: "Email Address".to_string(),
                field_type: FieldType::Email,
                required: true,
                validation_rules: ValidationRules {
                    regex: None,
                    custom_error_message: Some("Please enter a valid email".to_string()),
                },
            },
            FieldDefinition {
                id: "name".to_string(),
                name: "Full Name".to_string(),
                field_type: FieldType::Text { max_length: Some(100) },
                required: false,
                validation_rules: ValidationRules {
                    regex: None,
                    custom_error_message: None,
                },
            },
        ];

        Self::new("Newsletter Signup".to_string(), developer_id, public_key, fields)
    }

    pub fn contact_form(developer_id: Uuid, public_key: String) -> Self {
        let fields = vec![
            FieldDefinition {
                id: "name".to_string(),
                name: "Name".to_string(),
                field_type: FieldType::Text { max_length: Some(100) },
                required: true,
                validation_rules: ValidationRules {
                    regex: None,
                    custom_error_message: None,
                },
            },
            FieldDefinition {
                id: "email".to_string(),
                name: "Email".to_string(),
                field_type: FieldType::Email,
                required: true,
                validation_rules: ValidationRules {
                    regex: None,
                    custom_error_message: None,
                },
            },
            FieldDefinition {
                id: "message".to_string(),
                name: "Message".to_string(),
                field_type: FieldType::Text { max_length: Some(2000) },
                required: true,
                validation_rules: ValidationRules {
                    regex: None,
                    custom_error_message: None,
                },
            },
        ];

        Self::new("Contact Form".to_string(), developer_id, public_key, fields)
    }
}

impl FormSubmission {
    /// Get the form schema this submission belongs to
    /// TODO: Implement database query for form schema
    pub async fn get_form_schema(&self) -> Result<FormSchema, FormVaultError> {
        unimplemented!("Query form_schemas table")
    }
    
    /// Retry webhook delivery
    /// TODO: Consider adding explicit handling for None webhook_url
    pub async fn retry_webhook(&mut self) -> Result<(), FormVaultError> {
        let schema = self.get_form_schema().await?;
        if let Some(webhook_url) = &schema.webhook_url {
            match send_webhook(self, webhook_url).await {
                Ok(_) => {
                    self.status = SubmissionStatus::Delivered;
                    update_submission_status(self).await?;
                }
                Err(e) => {
                    self.status = SubmissionStatus::Failed(e.to_string());
                    update_submission_status(self).await?;
                }
            }
        }
        Ok(())
    }
}

impl Form for FormSubmission {
    fn form_id(&self) -> Uuid {
        self.id
    }
    
    fn form_type(&self) -> FormType {
        FormType::Custom("submission".to_string())
    }
    
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    /// TODO: Implement database deletion logic
    fn delete(self) -> Result<(), FormVaultError> {
        unimplemented!("Database deletion logic")
    }
}
