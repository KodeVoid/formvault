use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FieldDefinition {
    pub id: Uuid,
    pub form_id: Uuid,
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub validation_rules: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "form_field_type")]
pub enum FieldType {
    Text,
    Email,
    Phone,
    Number,
    Date,
    Select,
    Checkbox,
    File,
}
