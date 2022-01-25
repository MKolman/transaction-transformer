import csv
import io
import logging
from transaction import Transaction

logger = logging.getLogger(__name__)


def write(store: io.TextIOWrapper, transactions: list[Transaction]):
    logger.info(
        "Saving {num_transactions} transactions into {storename}.",
        num_transactions=len(transactions),
        storename=getattr(store, "name", "buffer"),
    )
    writer = csv.DictWriter(
        store,
        [
            "Date",
            "Account",
            "Transaction Account",
            "Deposit",
            "Withdrawal",
            "Description",
        ],
    )
    writer.writeheader()
    for trans in transactions:
        writer.writerow(
            {
                "Date": trans.date,
                "Account": trans.debtor_account,
                "Transaction Account": trans.creditor_account,
                "Deposit": trans.debit,
                "Withdrawal": trans.credit,
                "Description": trans.description,
            }
        )
