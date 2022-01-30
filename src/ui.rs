pub mod string;
pub mod terminal;

use crate::matcher::Candidate;

pub trait UI {
    fn choose_or_create_match(&self, account: &str, candidates: &Vec<Candidate>) -> String;
}
