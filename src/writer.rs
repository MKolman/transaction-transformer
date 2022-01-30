use crate::transaction::Transaction;
use log::info;

pub fn write(store: impl std::io::Write, transactions: &[Transaction]) -> Result<(), csv::Error> {
    info!(
        "Saving {num_transactions} transactions into {storename}",
        num_transactions = transactions.len(),
        storename = "store",
    );
    let mut writer = csv::Writer::from_writer(store);
    for transaction in transactions {
        writer.serialize(transaction)?;
    }
    writer.flush()?;
    Ok(())
}
