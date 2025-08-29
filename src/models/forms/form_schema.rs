use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{SubmissionMetadata, FormSubmission};
use crate::errors::FormVaultError;
use crate::repositories::{encryption::encrypt_form_data, form::{save_submission, update_submission_status}};
use crate::webhook::send_webhook;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FormSchema {
    pub id: Uuid,
    pub name: String,
    pub developer_id: Uuid,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
    pub webhook_url: Option<String>,
}

impl FormSchema {
    pub fn new(name: String, developer_id: Uuid, public_key: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            developer_id,
            public_key,
            created_at: Utc::now(),
            webhook_url: None,
        }
    }

    pub async fn process_submission(
        &self,
        raw_data: std::collections::HashMap<String, String>,
        metadata: SubmissionMetadata,
    ) -> Result<FormSubmission, FormVaultError> {
        let (encrypted_data, encrypted_key) =
            encrypt_form_data(&raw_data, &self.public_key).await?;

        let mut submission = FormSubmission::new(self.id, encrypted_data, encrypted_key, metadata);

        save_submission(&submission).await?;

        if let Some(url) = &self.webhook_url {
            match send_webhook(&submission, url).await {
                Ok(_) => submission.mark_delivered().await?,
                Err(e) => submission.mark_failed(e.to_string()).await?,
            }
        }

        Ok(submission)
    }
}
