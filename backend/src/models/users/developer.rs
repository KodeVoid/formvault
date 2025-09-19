use crate::errors::{FormVaultError, FormVaultResult};
use crate::models::forms::form_schema::FormSchema;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Developer {
    name: String,
    email: String,
    public_key: String,
    id: Uuid,
    created_at: DateTime<Utc>,
    api_key: String,
    is_active: bool,
}

impl Developer {
    /// Create a new developer
    pub fn new(name: String, email: String, public_key: String) -> Self {
        Developer {
            name,
            email,
            public_key,
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            api_key: Self::generate_api_key(),
            is_active: true,
        }
    }

    /// Generate a secure API key for the developer
    fn generate_api_key() -> String {
        format!("fv_{}", Uuid::new_v4().simple())
    }

    /// Save developer to database
    pub async fn save(&self, pool: &PgPool) -> FormVaultResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO developers (id, name, email, public_key, created_at, api_key, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            self.id,
            self.name,
            self.email,
            self.public_key,
            self.created_at,
            self.api_key,
            self.is_active
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Find developer by ID
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> FormVaultResult<Option<Self>> {
        let developer = sqlx::query_as!(
            Developer,
            "SELECT id, name, email, public_key, created_at, api_key, is_active FROM developers WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(developer)
    }

    /// Find developer by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> FormVaultResult<Option<Self>> {
        let developer = sqlx::query_as!(
            Developer,
            "SELECT id, name, email, public_key, created_at, api_key, is_active FROM developers WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(developer)
    }

    /// Find developer by API key (for authentication)
    pub async fn find_by_api_key(api_key: &str, pool: &PgPool) -> FormVaultResult<Option<Self>> {
        let developer = sqlx::query_as!(
            Developer,
            "SELECT id, name, email, public_key, created_at, api_key, is_active FROM developers WHERE api_key = $1 AND is_active = true",
            api_key
        )
        .fetch_optional(pool)
        .await?;

        Ok(developer)
    }

    /// Update developer information
    pub async fn update(&mut self, pool: &PgPool) -> FormVaultResult<()> {
        sqlx::query!(
            r#"
            UPDATE developers 
            SET name = $1, email = $2, public_key = $3
            WHERE id = $4
            "#,
            self.name,
            self.email,
            self.public_key,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Regenerate API key (in case of compromise)
    pub async fn regenerate_api_key(&mut self, pool: &PgPool) -> FormVaultResult<String> {
        let new_api_key = Self::generate_api_key();

        sqlx::query!(
            "UPDATE developers SET api_key = $1 WHERE id = $2",
            new_api_key,
            self.id
        )
        .execute(pool)
        .await?;

        self.api_key = new_api_key.clone();
        Ok(new_api_key)
    }

    /// Deactivate developer account
    pub async fn deactivate(&mut self, pool: &PgPool) -> FormVaultResult<()> {
        self.is_active = false;

        sqlx::query!(
            "UPDATE developers SET is_active = false WHERE id = $1",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get all forms belonging to this developer
    pub async fn get_forms(&self, pool: &PgPool) -> FormVaultResult<Vec<FormSchema>> {
        let forms = sqlx::query_as!(
            FormSchema,
            r#"
            SELECT id, name, developer_id, public_key, created_at, webhook_url
            FROM form_schemas 
            WHERE developer_id = $1 
            ORDER BY created_at DESC
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(forms)
    }

    /// Get submission count across all developer's forms
    // pub async fn get_total_submissions(&self, pool: &PgPool) -> FormVaultResult<i64> {
    //     let count = sqlx::query!(
    //         r#"
    //         SELECT COUNT(*) as count
    //         FROM form_submissions fs
    //         JOIN form_schemas sc ON fs.form_schema_id = sc.id
    //         WHERE sc.developer_id = $1
    //         "#,
    //         self.id
    //     )
    //     .fetch_one(pool)
    //     .await?
    //     .count;

    //     Ok(count)
    // }

    // /// Validate API key for requests
    pub async fn authenticate(api_key: &str, pool: &PgPool) -> FormVaultResult<Self> {
        match Self::find_by_api_key(api_key, pool).await? {
            Some(developer) => Ok(developer),
            None => Err(FormVaultError::Unauthorized),
        }
    }

    /// Update public key (when developer rotates keys)
    pub async fn update_public_key(
        &mut self,
        new_public_key: String,
        pool: &PgPool,
    ) -> FormVaultResult<()> {
        // TODO: Validate the public key format
        if !Self::is_valid_public_key(&new_public_key) {
            return Err(FormVaultError::InvalidPublicKey);
        }

        self.public_key = new_public_key;

        sqlx::query!(
            "UPDATE developers SET public_key = $1 WHERE id = $2",
            self.public_key,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Validate public key format (placeholder - implement actual validation)
    fn is_valid_public_key(key: &str) -> bool {
        // TODO: Implement proper RSA public key validation
        !key.is_empty() && key.len() > 100
    }

    /// Check if email is already taken
    pub async fn email_exists(email: &str, pool: &PgPool) -> FormVaultResult<bool> {
        let record = sqlx::query!(
            "SELECT COUNT(*) as count FROM developers WHERE email = $1",
            email
        )
        .fetch_one(pool)
        .await?;

        if let Some(count) = record.count {
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }

    /// Create developer with email uniqueness check
    pub async fn create_unique(
        name: String,
        email: String,
        public_key: String,
        pool: &PgPool,
    ) -> FormVaultResult<Self> {
        // Check if email already exists
        if Self::email_exists(&email, pool).await? {
            return Err(FormVaultError::DuplicateEmail);
        }

        // Validate public key
        if !Self::is_valid_public_key(&public_key) {
            return Err(FormVaultError::InvalidPublicKey);
        }

        let developer = Self::new(name, email, public_key);
        developer.save(pool).await?;

        Ok(developer)
    }

    // Getters for private fields
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
