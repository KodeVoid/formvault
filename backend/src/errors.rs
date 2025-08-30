use std::fmt;
use reqwest::*;
use serde::{Deserialize, Serialize};

// Single unified error type for the entire application
#[derive(Debug)]
pub enum FormVaultError {
    // Database errors
    DatabaseError(sqlx::Error),
    
    // Resource errors
    NotFound,
    DeveloperNotFound,
    FormNotFound,
    SubmissionNotFound,
    
    // Authentication/Authorization
    Unauthorized,
    InvalidApiKey,
    
    // Validation errors
    ValidationFailed(Vec<String>),
    ValidationError(String),
    InvalidEmail,
    InvalidPublicKey,
    DuplicateEmail,
    
    // Encryption errors
    EncryptionError(String),
    DecryptionError(String),
    
    // Webhook errors
    WebhookFailed(String),
    WebhookTimeout,
    
    // Network errors
    NetworkError(String),
    
    // Business logic errors
    FormLimitExceeded,
    SubmissionLimitExceeded,
    InactiveAccount,

}

impl fmt::Display for FormVaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
         {
            FormVaultError::DatabaseError(e) => {
                write!(f, "Database error: {}", e)
            }
            FormVaultError::NotFound => {
                write!(f, "Resource not found")
            }
            FormVaultError::DeveloperNotFound => {
                write!(f, "Developer account not found")
            }
            FormVaultError::FormNotFound => {
                write!(f, "Form not found")
            }
            FormVaultError::SubmissionNotFound => {
                write!(f, "Form submission not found")
            }
            FormVaultError::Unauthorized => {
                write!(f, "Unauthorized access")
            }
            FormVaultError::InvalidApiKey => {
                write!(f, "Invalid or expired API key")
            }
            FormVaultError::ValidationFailed(errors) => {
                write!(f, "Validation failed: {}", errors.join(", "))
            }
            FormVaultError::InvalidEmail => {
                write!(f, "Invalid email address format")
            }
            FormVaultError::InvalidPublicKey => {
                write!(f, "Invalid public key format")
            }
            FormVaultError::DuplicateEmail => {
                write!(f, "Email address already registered")
            }
            FormVaultError::EncryptionError(msg) => {
                write!(f, "Encryption error: {}", msg)
            }
            FormVaultError::DecryptionError(msg) => {
                write!(f, "Decryption error: {}", msg)
            }
            FormVaultError::WebhookFailed(msg) => {
                write!(f, "Webhook delivery failed: {}", msg)
            }
            FormVaultError::WebhookTimeout => {
                write!(f, "Webhook request timed out")
            }
            FormVaultError::NetworkError(msg) => {
                write!(f, "Network error: {}", msg)
            }
            FormVaultError::FormLimitExceeded => {
                write!(f, "Maximum number of forms exceeded for your plan")
            }
            FormVaultError::SubmissionLimitExceeded => {
                write!(f, "Monthly submission limit exceeded")
            }
            FormVaultError::InactiveAccount => {
                write!(f, "Account is inactive or suspended")
            }
            FormVaultError::ValidationError(e)=>{
                write!(f,"Validation Failed {}",e)
            }
        }
    }
}

impl std::error::Error for FormVaultError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FormVaultError::DatabaseError(e) => Some(e),
            _ => None,
        }
    }
}

// Convert from sqlx::Error
impl From<sqlx::Error> for FormVaultError {
    fn from(err: sqlx::Error) -> Self {
        FormVaultError::DatabaseError(err)
    }
}

// Convert from reqwest::Error (for webhook calls)
impl From<reqwest::Error> for FormVaultError {
    fn from(err: reqwest::Error) -> Self {
        FormVaultError::NetworkError(err.to_string())
    }
}

// Result type alias for convenience
pub type FormVaultResult<T> = std::result::Result<T, FormVaultError>;

// Error response for API endpoints
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub details: Option<Vec<String>>,
}

impl FormVaultError {
    /// Convert error to HTTP response format
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            FormVaultError::NotFound | FormVaultError::DeveloperNotFound 
            | FormVaultError::FormNotFound | FormVaultError::SubmissionNotFound => {
                ErrorResponse {
                    error: self.to_string(),
                    code: "NOT_FOUND".to_string(),
                    details: None,
                }
            }
            FormVaultError::Unauthorized | FormVaultError::InvalidApiKey => {
                ErrorResponse {
                    error: "Access denied".to_string(),
                    code: "UNAUTHORIZED".to_string(),
                    details: None,
                }
            }
            FormVaultError::ValidationFailed(errors) => {
                ErrorResponse {
                    error: "Validation failed".to_string(),
                    code: "VALIDATION_ERROR".to_string(),
                    details: Some(errors.clone()),
                }
            }
            FormVaultError::DuplicateEmail => {
                ErrorResponse {
                    error: self.to_string(),
                    code: "DUPLICATE_EMAIL".to_string(),
                    details: None,
                }
            }
            FormVaultError::FormLimitExceeded | FormVaultError::SubmissionLimitExceeded => {
                ErrorResponse {
                    error: self.to_string(),
                    code: "LIMIT_EXCEEDED".to_string(),
                    details: None,
                }
            }
            _ => {
                ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                    details: None,
                }
            }
        }
    }

    /// Get HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            FormVaultError::NotFound | FormVaultError::DeveloperNotFound 
            | FormVaultError::FormNotFound | FormVaultError::SubmissionNotFound => 404,
            
            FormVaultError::Unauthorized | FormVaultError::InvalidApiKey => 401,
            
            FormVaultError::ValidationFailed(_) | FormVaultError::InvalidEmail 
            | FormVaultError::InvalidPublicKey | FormVaultError::DuplicateEmail => 400,
            
            FormVaultError::FormLimitExceeded | FormVaultError::SubmissionLimitExceeded => 429,
            
            FormVaultError::InactiveAccount => 403,
            
            _ => 500,
        }
    }
}