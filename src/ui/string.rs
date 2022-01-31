use crate::matcher::Candidate;

pub struct UI(pub String);

impl super::UI for UI {
    fn choose_or_create_match(&self, _: &str, _: &[Candidate]) -> String {
        self.0.clone()
    }
}
