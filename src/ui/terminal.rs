use std::io::Write;

use crate::matcher::Candidate;
pub struct UI;

impl UI {
    fn ask_user_to_pick(account: &str, candidates: &[Candidate]) -> String {
        println!("\n=== {account} ===", account = account);
        if candidates.is_empty() {
            print!("Enter account: ");
        } else {
            for (i, candidate) in candidates.iter().enumerate() {
                println!(
                    "{i}. {score:.1}% ({to}) {from}",
                    i = i + 1,
                    score = candidate.score * 100.0,
                    to = candidate.translates_to,
                    from = candidate.similar_to,
                );
            }
            print!("[1-{len}] or new: ", len = candidates.len());
        }
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim_end().to_owned()
    }
}
impl super::UI for UI {
    fn choose_or_create_match(&self, account: &str, candidates: &[Candidate]) -> String {
        let value = Self::ask_user_to_pick(account, candidates);
        match value.parse::<usize>() {
            Ok(idx) if 0 < idx && idx <= candidates.len() => {
                candidates[idx - 1].translates_to.clone()
            }
            _ => value,
        }
    }
}
