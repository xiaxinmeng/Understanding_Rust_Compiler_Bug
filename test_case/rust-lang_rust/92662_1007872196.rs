bash
cargo bisect-rustc --preserve --start=2022-01-06 --end=2022-01-07 --regress=error -- doc
