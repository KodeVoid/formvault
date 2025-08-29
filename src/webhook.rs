use crate::errors::FormVaultError;
use crate::models::form::FormSubmission;
/// TODO: Implement HTTP webhook call using `reqwest`
pub async fn send_webhook(submission: &FormSubmission, webhook_url: &str) -> Result<(), FormVaultError> {
    unimplemented!("HTTP webhook call")
}