mod score;
#[cfg(test)]
mod tests;

use crate::ui::UI;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Candidate<'a> {
    pub score: f64,
    pub similar_to: &'a String,
    pub translates_to: &'a String,
}

impl<'a> From<(f64, &'a String, &'a String)> for Candidate<'a> {
    fn from((score, similar_to, translates_to): (f64, &'a String, &'a String)) -> Self {
        Candidate {
            score,
            similar_to,
            translates_to,
        }
    }
}

impl<'a> Eq for Candidate<'a> {}

impl<'a> Ord for Candidate<'a> {
    fn cmp(&self, other: &Candidate) -> std::cmp::Ordering {
        self.score
            .partial_cmp(&other.score)
            .expect("score is an invalid floating point number")
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountMatcher {
    accounts: HashMap<String, String>,
}

impl AccountMatcher {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
    pub fn from_path(path: &str) -> Result<Self, csv::Error> {
        let mut reader = csv::Reader::from_path(path)?;
        let accounts: HashMap<String, String> = HashMap::from_iter(
            reader
                .deserialize()
                .collect::<Result<Vec<(String, String)>, csv::Error>>()?,
        );
        Ok(Self::from(accounts))
    }
    pub fn from(accounts: HashMap<String, String>) -> Self {
        Self { accounts }
    }
    pub fn find_match(&mut self, account: &str, ui: &impl UI) -> String {
        let candidates: Vec<Candidate> = self
            .accounts
            .iter()
            .map(|(acc, matched)| (score::score(account, acc), acc, matched).into())
            .filter(|cand: &Candidate| cand.score > 0.5)
            .collect();
        let best = candidates.iter().max();
        let result = match best {
            Some(candidate) if candidate.score > 0.75 => {
                info!(
                    "Automatically matched '{matched}' to '{account}', ({score:.1}% match to '{acc}')",
                    account = account,
                    acc = candidate.similar_to,
                    matched = candidate.translates_to,
                    score = candidate.score * 100.0
                );
                candidate.translates_to.clone()
            }
            _ => ui.choose_or_create_match(account, &candidates),
        };
        self.accounts.insert(account.to_owned(), result.clone());
        result
    }
}
