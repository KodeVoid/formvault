use serde::{Deserialize, Serialize};

use super::traits::{Identifiable, User};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StaffRole {
    Admin,
    Support,
    Reviewer,
    Manager,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub base: User,
    pub role: StaffRole,
    pub permissions: Vec<String>,
    // more staff-specific fields
}

impl Identifiable for Staff {
    fn email(&self) -> &str {
        &self.base.email
    }
    fn id(&self) -> uuid::Uuid {
        self.base.id
    }
}
