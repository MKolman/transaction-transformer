import unittest
import io

import writer
import transaction


class TestTransactionWriter(unittest.TestCase):
    def test_empty(self):
        csv = io.StringIO()
        writer.write(csv, [])
        self.assertIn(
            "Date,Account,Transaction Account,Deposit,Withdrawal,Description",
            csv.getvalue(),
        )

    def test_double(self):
        csv = io.StringIO()
        writer.write(
            csv,
            [
                transaction.Transaction(
                    "1994", "ACCIN", "ACCOUT", "DESC", "99.15", "0.0"
                ),
                transaction.Transaction(
                    "1994", "ACCIN", "ACCOUT", "DESC", "99.15", "0.0"
                ),
            ],
        )
        self.assertIn("\r\n1994,ACCIN,ACCOUT,99.15,0.0,DESC" * 2, csv.getvalue())
