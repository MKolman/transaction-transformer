import csv
import io
import logging
from transaction import Transaction

logger = logging.getLogger(__name__)


def write(store: io.TextIOWrapper, transactions: list[Transaction]):
    logger.info(
        f"Saving {len(transactions)} transactions into {getattr(store, 'name', 'buffer')}."
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
