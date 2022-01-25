import unittest
import io

import reader
from transaction import Transaction


class TestValidator(unittest.TestCase):
    def test_success(self):
        config = reader.ReadConfig(
            "date",
            ["in_acc"],
            ["place holder"],
            ["place holder"],
            "place holder",
            "place holder",
            True,
        )
        reader.validate(["date", "in_acc", "place holder"], config)

    def test_fail(self):
        config = reader.ReadConfig(
            "date",
            ["in_acc"],
            ["out_acc"],
            ["desc"],
            "debit",
            "credit",
            True,
        )
        self.assertRaisesRegex(
            AssertionError,
            "Missing columns {(?=.*date)(?=.*in_acc)(?=.*out_acc)(?=.*desc)(?=.*debit)(?=.*credit).*}",
            lambda: reader.validate([], config),
        )


class TestReader(unittest.TestCase):
    def test_empty(self):
        table = io.StringIO("1,2,3,4,5")
        config = reader.ReadConfig(
            "1",
            ["2"],
            ["3"],
            ["4"],
            "5",
            "5",
            True,
        )
        self.assertEqual([], reader.validate_and_read(table, config))

    def test_abs(self):
        table = io.StringIO("1,2,3,4,5\na,b,c,d,e\nx,y,z,w,-q")
        config = reader.ReadConfig(
            "1",
            ["2"],
            ["3"],
            ["4"],
            "5",
            "5",
            True,
        )
        transactions = reader.validate_and_read(table, config)
        want = [
            Transaction("a", "b", "c", "d", "e", "e"),
            Transaction("x", "y", "z", "w", "q", "q"),
        ]
        self.assertListEqual(want, transactions)
