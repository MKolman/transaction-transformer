use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Transaction {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Account")]
    pub debtor_account: String,
    #[serde(rename = "Transaction Account")]
    pub creditor_account: String,
    #[serde(rename = "Deposit")]
    pub debit: String,
    #[serde(rename = "Withdrawal")]
    pub credit: String,
    #[serde(rename = "Description")]
    pub description: String,
}
