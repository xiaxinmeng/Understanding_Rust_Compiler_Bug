
cargo-bisect-rustc --start 2019-08-13 --end $(date --iso) --preserve --regress=ice -- check
