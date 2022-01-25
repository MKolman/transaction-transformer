import csv
import io
import logging
from dataclasses import dataclass
from typing import Iterable

from transaction import Transaction

logger = logging.getLogger(__name__)


@dataclass
class ReadConfig:
    date_field: str
    debtor_fields: list[str]
    creditor_fields: list[str]
    description_fields: list[str]
    debit_field: str
    credit_field: str
    amount_abs: bool


def validate_and_read(store: io.TextIOWrapper, config: ReadConfig) -> list[Transaction]:
    transaction_data = csv.DictReader(store, delimiter=",", quotechar='"')
    validate(transaction_data.fieldnames, config)
    transactions = read(transaction_data, config)
    logger.info(
        f"Read {len(transactions)} transactions from {getattr(store, 'name', 'buffer')}"
    )
    return transactions


def read(data: Iterable[dict[str, str]], config: ReadConfig) -> list[Transaction]:
    to_amount = lambda a: a
    if config.amount_abs:
        to_amount = lambda a: a.lstrip(" -")
    result = []
    for row in data:
        result.append(
            Transaction(
                date=row[config.date_field],
                debtor_account=" | ".join(map(row.get, config.debtor_fields)),
                creditor_account=" | ".join(map(row.get, config.creditor_fields)),
                description=" | ".join(map(row.get, config.description_fields)),
                debit=to_amount(row[config.debit_field]),
                credit=to_amount(row[config.credit_field]),
            )
        )
    return result


def validate(all_fields: Iterable[str], config: ReadConfig):
    required_fields = set(
        config.debtor_fields
        + config.creditor_fields
        + config.description_fields
        + [config.date_field, config.debit_field, config.credit_field]
    )
    missing_columns = required_fields - set(all_fields)
    assert (
        len(missing_columns) == 0
    ), f"Missing columns {missing_columns} in {all_fields}"
