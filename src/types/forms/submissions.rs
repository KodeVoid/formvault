use super::traits::Form;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Submission<T>
where
    T: Form,
{
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: T,
}
impl<T: Form> Submission<T> {
    pub fn new(data: T) -> Self {
        Submission {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            data,
        }
    }
}
