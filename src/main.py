import argparse
import csv
import logging
import sys

import matcher
import reader
import writer

arg_parser = argparse.ArgumentParser(
    description="Transforms transaction CSV into GNUCash format."
)

arg_parser.add_argument(
    "infile",
    nargs="?",
    type=argparse.FileType("r"),
    default=sys.stdin,
)
arg_parser.add_argument(
    "outfile",
    nargs="?",
    type=argparse.FileType("w"),
    default=sys.stdout,
)
arg_parser.add_argument("--matchfile", nargs="?")
arg_parser.add_argument("--debtor_col", nargs="+", required=True)
arg_parser.add_argument("--creditor_col", nargs="+", required=True)
arg_parser.add_argument("--date_col", required=True)
arg_parser.add_argument("--desc_col", nargs="+", required=True)
arg_parser.add_argument("--debit_col", required=True)
arg_parser.add_argument("--credit_col", required=True)
arg_parser.add_argument(
    "--amount_abs",
    action="store_const",
    const=True,
    default=False,
)
arg_parser.add_argument(
    "--log",
    choices=["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"],
    default="WARNING",
    type=str.upper,
)


def main():
    args = arg_parser.parse_args()
    logging.basicConfig(level=getattr(logging, args.log))
    read_config = reader.ReadConfig(
        date_field=args.date_col,
        debtor_fields=args.debtor_col,
        creditor_fields=args.creditor_col,
        description_fields=args.desc_col,
        debit_field=args.debit_col,
        credit_field=args.credit_col,
        amount_abs=args.amount_abs,
    )

    transactions = reader.validate_and_read(args.infile, read_config)

    transformer = matcher.AccountMatcher.load(args.matchfile)
    for transaction in transactions:
        transaction.debtor_account = transformer.find_match(transaction.debtor_account)
        transaction.creditor_account = transformer.find_match(
            transaction.creditor_account
        )

    writer.write(args.outfile, transactions)
    if args.matchfile:
        transformer.dump(args.matchfile)


if __name__ == "__main__":
    main()
