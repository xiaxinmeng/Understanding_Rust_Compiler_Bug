bash
cargo bisect-rustc --access=github --start=1.66.1 --end=2022-12-10 --regress=non-ice -- build
