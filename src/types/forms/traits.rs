pub trait Form {
    fn form_type(&self) -> FormType;
}

#[derive(Debug)]
pub enum FormType {
    NewsLetter,
}
