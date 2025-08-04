use super::{
    submissions::Submission,
    traits::{Form, FormType},
};

#[derive(Debug)]
pub struct FormEntry<T: Form> {
    pub form_type: FormType,
    pub submissions: Vec<Submission<T>>,
}

// Correct `impl` syntax
impl<T: Form> FormEntry<T> {
    pub fn new(form_type: FormType) -> Self {
        Self {
            form_type,
            submissions: Vec::new(),
        }
    }

    pub fn add_submission(&mut self, data: T) {
        let submission = Submission::new(data);
        self.submissions.push(submission);
    }

    pub fn all(&self) -> &[Submission<T>] {
        &self.submissions
    }
}
