use crate::repositories::form::update_submission_status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FormSubmission {
    pub id: Uuid,
    pub form_schema_id: Uuid,
    pub encrypted_data: String,
    pub encrypted_key: String,
    pub metadata: SubmissionMetadata,
    pub created_at: DateTime<Utc>,
    pub status: SubmissionStatus,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "submission_status")]
pub enum SubmissionStatus {
    New,
    Processing,
    Delivered,
    Failed,
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub country: Option<String>,
}

impl FormSubmission {
    pub fn new(
        form_schema_id: Uuid,
        encrypted_data: String,
        encrypted_key: String,
        metadata: SubmissionMetadata,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            form_schema_id,
            encrypted_data,
            encrypted_key,
            metadata,
            created_at: Utc::now(),
            status: SubmissionStatus::New,
            failure_reason: None,
        }
    }

    pub async fn mark_delivered(&mut self) -> Result<(), crate::errors::FormVaultError> {
        self.status = SubmissionStatus::Delivered;
        self.failure_reason = None;
        update_submission_status(self).await
    }

    pub async fn mark_failed(
        &mut self,
        reason: String,
    ) -> Result<(), crate::errors::FormVaultError> {
        self.status = SubmissionStatus::Failed;
        self.failure_reason = Some(reason);
        update_submission_status(self).await
    }
}
