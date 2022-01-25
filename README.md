# Transaction transformer

This repository contains a script that can be used to transform
transations in almost arbitrary format into a format that can be
understood by GNUCash.

## Example usage
```
python src/main.py \
    --date_col="Posted Transactions Date" \
    --debtor_col="Transfer Account" \
    --creditor_col="Posted Account" \
    --desc_col="Description" \
    --debit_col="Debit Amount" \
    --credit_col="Credit Amount" \
    --log=INFO \
    --matchfile=matches.json \
    -- \
    input_statement.csv \
    output_statement.csv
```