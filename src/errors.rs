use std::fmt;

#[derive(Debug)]
pub enum FormVaultError {
    DatabaseError(String),
    NotFound,
    Unauthorized,
    ValidationFailed(Vec<String>),
}

impl fmt::Display for FormVaultError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            FormVaultError::DatabaseError(s) => {
                write!(formatter, "There is an error in the database: {}", s)
            }
            FormVaultError::NotFound => write!(formatter, "Resource not found"),
            FormVaultError::Unauthorized => write!(formatter, "Unauthorized access"),
            FormVaultError::ValidationFailed(errors) => {
                write!(formatter, "Validation failed: {}", errors.join(", "))
            }
        }
    }
}
