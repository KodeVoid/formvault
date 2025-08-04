use super::traits::{Form, FormType};
use crate::types::encryption::encrypted_string::EncryptedString;

pub struct NewsLetter {
    name: EncryptedString,
    email: EncryptedString,
}
#[allow(dead_code)]
impl NewsLetter {
    pub fn new(name: &str, email: &str) -> Self {
        NewsLetter {
            name: EncryptedString::new(name),
            email: EncryptedString::new(email),
        }
    }
}

impl Form for NewsLetter {
    fn form_type(&self) -> FormType {
        FormType::NewsLetter
    }
}
