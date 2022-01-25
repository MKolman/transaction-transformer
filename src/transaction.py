from dataclasses import dataclass


@dataclass
class Transaction:
    date: str
    debtor_account: str
    creditor_account: str
    description: str
    debit: str
    credit: str
