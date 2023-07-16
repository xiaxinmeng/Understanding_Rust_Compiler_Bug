bash
cargo bisect-rustc --preserve --access github --start 1.66.0 --regress=error --end 2022-12-08 -- check
