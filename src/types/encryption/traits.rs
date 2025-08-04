pub trait Encrypted {
    fn raw(&self) -> &[u8];
    fn content_type(&self) -> &'static str;
    fn encrypter(&self) -> Encrypter;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encrypter {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}
