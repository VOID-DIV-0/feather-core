table read 'data.csv' delimiter ',' header true into ::rows.
table write ::rows to 'out.tsv' delimiter '\t'.
