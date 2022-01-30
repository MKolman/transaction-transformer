use crate::matcher::Candidate;

pub struct StringUI(pub String);

impl super::UI for StringUI {
    fn choose_or_create_match(&self, _: &str, _: &[Candidate]) -> String {
        self.0.clone()
    }
}
