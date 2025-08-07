use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::traits::{Identifiable, User};
use sqlx::{PgPool, Result};

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
    pub base: User,                  // already created user
    pub role: StaffRole,
    pub permissions: Vec<String>,
}

impl Identifiable for Staff {
    fn id(&self) -> Uuid {
        self.base.id
    }

    fn email(&self) -> &str {
        &self.base.email
    }
}

impl Staff {
    pub fn new(base: User, role: StaffRole, permissions: Vec<String>) -> Self {
        Staff { base, role, permissions }
    }

    /// Insert only staff-specific data. Assumes `User` is already inserted.
    pub async fn create_staff(&self, pool: &PgPool) -> Result<Uuid> {
        let role_string = match &self.role {
            StaffRole::Admin => "Admin".to_string(),
            StaffRole::Support => "Support".to_string(),
            StaffRole::Reviewer => "Reviewer".to_string(),
            StaffRole::Manager => "Manager".to_string(),
            StaffRole::Custom(s) => format!("Custom:{}", s),
        };

        let permissions_json = serde_json::to_string(&self.permissions)?;

        sqlx::query!(
            "INSERT INTO staff (user_id, role, permissions) VALUES ($1, $2, $3)",
            self.base,
            role_string,
            permissions_json,
        )
        .execute(pool)
        .await?;

        Ok(self.base.id)
    }
}
