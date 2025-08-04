use super::traits::{Encrypted, Encrypter};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EncryptedString(pub String);
impl EncryptedString {
    pub fn new(string: &str) -> Self {
        EncryptedString(string.to_owned())
    }
}

impl Encrypted for EncryptedString {
    fn raw(&self) -> &[u8] {
        self.0.as_bytes()
    }

    fn content_type(&self) -> &'static str {
        "an encrypted string"
    }

    fn encrypter(&self) -> Encrypter {
        Encrypter {
            name: "form_vault_string_encrypter",
            version: "v1",
            description: "Takes an input plaintext and uses a private key to hash it. Comes with an associated decrypter pair.",
        }
    }
}
