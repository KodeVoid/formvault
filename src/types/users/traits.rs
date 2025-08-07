use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
// Domain trait and struct (your business logic layer)
pub trait Identifiable {
    fn id(&self) -> Uuid;
    fn email(&self) -> &str;
}

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl Identifiable for User {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn email(&self) -> &str {
        &self.email
    }
}


// For new users (when you don't have an ID yet)
impl User {
    pub fn new(email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            created_at: Utc::now(),
        }
    }
    
}

// Sql
