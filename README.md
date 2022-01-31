![Build, test and lint](https://github.com/MKolman/transaction-transformer/actions/workflows/build.yml/badge.svg)

# Transaction transformer

This repository contains a script that can be used to transform
transations in almost arbitrary format into a format that can be
understood by GNUCash.

## Example usage
```
cargo run -- \
    --date-col="Posted Transactions Date" \
    --debtor-col="Transfer Account,Description" \
    --creditor-col="Posted Account" \
    --desc-col="Description" \
    --debit-col="Debit Amount" \
    --credit-col="Credit Amount" \
    --log=INFO \
    --matchfile=matches.json \
    -- \
    input_statement.csv \
    output_statement.csv
```
