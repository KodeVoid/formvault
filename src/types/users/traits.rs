use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;
use anyhow::Result;
use time::OffsetDateTime;

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

impl User {
    pub fn new(email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            created_at: Utc::now(),
        }
    }
}

pub async fn create_user(pool: &PgPool, user: User) -> Result<User> {
    // Convert chrono::DateTime<Utc> to time::OffsetDateTime
    let created_at_odt = OffsetDateTime::from_unix_timestamp(user.created_at.timestamp())
        .unwrap()
        .replace_nanosecond(user.created_at.timestamp_subsec_nanos())
        .unwrap();

    sqlx::query!(
        "INSERT INTO users (id, email, created_at) VALUES ($1, $2, $3)",
        user.id,
        user.email,
        created_at_odt
    )
    .execute(pool)
    .await?;

    Ok(user)
}
