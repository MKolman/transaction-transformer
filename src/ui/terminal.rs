use std::collections::HashMap;
use std::io::Write;

use crate::matcher::Candidate;

type GroupedCandidates<'a, 'b> = (String, Vec<&'a Candidate<'b>>);

pub struct UI;

impl UI {
    fn group_candidates<'a, 'b>(candidates: &'a [Candidate<'b>]) -> Vec<GroupedCandidates<'a, 'b>> {
        let mut grouped: HashMap<String, Vec<_>> = HashMap::new();
        for candidate in candidates {
            grouped
                .entry(candidate.translates_to.clone())
                .or_default()
                .push(candidate);
        }
        grouped
            .values_mut()
            .for_each(|candidates| candidates.sort_unstable());
        grouped.into_iter().collect()
    }
    fn ask_user_to_pick(account: &str, candidates: &[GroupedCandidates]) -> String {
        println!("\n=== {account} ===", account = account);
        if candidates.is_empty() {
            print!("Enter account: ");
        } else {
            for (i, (account, candidates)) in candidates.iter().enumerate() {
                println!("{i}. {account}", i = i + 1, account = account,);
                for candidate in candidates {
                    println!(
                        "\t{score:.1}% {from}",
                        score = candidate.score * 100.0,
                        from = candidate.similar_to,
                    );
                }
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
        let candidates = Self::group_candidates(candidates);
        let value = Self::ask_user_to_pick(account, &candidates);
        match value.parse::<usize>() {
            Ok(idx) if 0 < idx && idx <= candidates.len() => candidates[idx - 1].0.clone(),
            _ => value,
        }
    }
}
