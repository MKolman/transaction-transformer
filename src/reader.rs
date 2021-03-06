use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadConfig {
    pub date_field: String,
    pub debtor_fields: Vec<String>,
    pub creditor_fields: Vec<String>,
    pub description_fields: Vec<String>,
    pub debit_field: String,
    pub credit_field: String,
    pub amount_abs: bool,
}

#[derive(Debug)]
pub enum ReadError {
    MissingColumn(HashSet<String>),
    Format(csv::ErrorKind),
}

impl From<csv::Error> for ReadError {
    fn from(err: csv::Error) -> Self {
        Self::Format(err.into_kind())
    }
}

impl PartialEq for ReadError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (ReadError::MissingColumn(a), ReadError::MissingColumn(b)) if a == b
        )
    }
}

pub fn validate_and_read(
    store: impl std::io::Read,
    config: &ReadConfig,
) -> Result<Vec<Transaction>, ReadError> {
    let mut reader = csv::Reader::from_reader(store);
    validate(reader.headers()?.into_iter(), config)?;
    let transactions = read(&mut reader, config)?;
    log::info!(
        "Read {num_transactions} transactions from {storename}",
        num_transactions = transactions.len(),
        storename = "store"
    );
    Ok(transactions)
}

type Row = HashMap<String, String>;

fn read<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
    config: &ReadConfig,
) -> Result<Vec<Transaction>, ReadError> {
    let join = |fields: &Vec<String>, row: &Row| {
        fields
            .iter()
            .map(|field| row[field].clone())
            .collect::<Vec<String>>()
            .join(" | ")
    };
    let to_amount = if config.amount_abs {
        |amount: String| amount.trim_start_matches(&['-', ' '] as &[_]).to_owned()
    } else {
        |amount: String| amount
    };
    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        let row: Row = result?;
        transactions.push(Transaction {
            date: row[&config.date_field].clone(),
            debtor_account: join(&config.debtor_fields, &row),
            creditor_account: join(&config.creditor_fields, &row),
            description: join(&config.description_fields, &row),
            debit: to_amount(row[&config.debit_field].clone()),
            credit: to_amount(row[&config.credit_field].clone()),
        });
    }
    Ok(transactions)
}

fn validate<'a>(data: impl Iterator<Item = &'a str>, config: &ReadConfig) -> Result<(), ReadError> {
    let mut fields = HashSet::new();
    fields.insert(config.date_field.clone());
    fields.extend(config.debtor_fields.iter().cloned());
    fields.extend(config.creditor_fields.iter().cloned());
    fields.extend(config.description_fields.iter().cloned());
    fields.insert(config.debit_field.clone());
    fields.insert(config.credit_field.clone());
    for col in data {
        fields.remove(col);
    }
    if fields.is_empty() {
        Ok(())
    } else {
        Err(ReadError::MissingColumn(fields))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_success() {
        let config = ReadConfig {
            date_field: "date".into(),
            creditor_fields: vec!["in_acc".into()],
            debtor_fields: vec!["place holder".into()],
            description_fields: vec!["place holder".into()],
            debit_field: "place holder".into(),
            credit_field: "place holder".into(),
            amount_abs: true,
        };
        assert!(validate(["date", "in_acc", "place holder"].into_iter(), &config).is_ok());
    }
    #[test]
    fn validate_fail() {
        let config = ReadConfig {
            date_field: "date".into(),
            creditor_fields: vec!["in_acc".into()],
            debtor_fields: vec!["out acc".into()],
            description_fields: vec!["desc".into()],
            debit_field: "debit".into(),
            credit_field: "credit".into(),
            amount_abs: true,
        };
        let got = validate([].into_iter(), &config);
        let want = Err(ReadError::MissingColumn(HashSet::from(
            ["date", "in_acc", "out acc", "desc", "debit", "credit"].map(String::from),
        )));
        assert_eq!(got, want);
    }

    #[test]
    fn read_empty() {
        let config = ReadConfig {
            date_field: "1".into(),
            creditor_fields: vec!["2".into()],
            debtor_fields: vec!["3".into()],
            description_fields: vec!["4".into()],
            debit_field: "5".into(),
            credit_field: "5".into(),
            amount_abs: true,
        };
        let store = b"1,2,3,4,5";
        let result = validate_and_read(&store[..], &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::new());
    }
    #[test]
    fn read_abs() {
        let config = ReadConfig {
            date_field: "1".into(),
            creditor_fields: vec!["2".into()],
            debtor_fields: vec!["3".into()],
            description_fields: vec!["4".into()],
            debit_field: "5".into(),
            credit_field: "5".into(),
            amount_abs: true,
        };
        let store = b"1,2,3,4,5\na,b,c,d,e\nx,y,z,w,-q";
        let result = validate_and_read(&store[..], &config);
        assert!(result.is_ok());
        let want = vec![
            Transaction {
                date: "a".into(),
                debtor_account: "c".into(),
                creditor_account: "b".into(),
                debit: "e".into(),
                credit: "e".into(),
                description: "d".into(),
            },
            Transaction {
                date: "x".into(),
                debtor_account: "z".into(),
                creditor_account: "y".into(),
                debit: "q".into(),
                credit: "q".into(),
                description: "w".into(),
            },
        ];
        assert_eq!(result.unwrap(), want);
    }
}
