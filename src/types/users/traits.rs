use serde::Serialize;
use uuid::Uuid;
pub trait Identifiable {
    fn id(&self) -> Uuid;
    fn email(&self) -> &str;
}

use chrono::{DateTime, Utc};
#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
