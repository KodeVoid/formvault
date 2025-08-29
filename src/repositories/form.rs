

use crate::errors::FormVaultError;

use crate::models::form::FormSubmission;

/// TODO: Implement database save using `sqlx` or `diesel`
pub async fn save_submission(submission: &FormSubmission) -> Result<(), FormVaultError> {
    unimplemented!("Database save")
}



/// TODO: Implement database update for submission status
pub async fn update_submission_status(submission: &FormSubmission) -> Result<(), FormVaultError> {
    unimplemented!("Database update")
}