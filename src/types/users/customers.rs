use super::traits::{Identifiable, User};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Customer {
    pub base: User,
    pub full_name: String,
    pub company: Option<String>,
    pub subscription_plan: String,
    pub form_entry_ids: Vec<Uuid>,
}
impl Identifiable for Customer {
    fn id(&self) -> Uuid {
        self.base.id
    }

    fn email(&self) -> &str {
        &self.base.email
    }
}
