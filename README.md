# RCSV : Sanitize CSV import file for Decidim participants

Check for unexpected chars in CSV

## Getting started

1. Compile program

```
cargo build --release
```

2. Execute program 

```
target/release/rcsv your_file.csv
```


### Notes

* CSV file must be comma separated (`,`)
* CSV must not contain header

