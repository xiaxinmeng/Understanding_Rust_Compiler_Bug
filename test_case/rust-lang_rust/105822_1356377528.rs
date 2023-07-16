bash
cargo bisect-rustc --preserve --access github --start 1.66.0 --regress=ice -- build --tests
