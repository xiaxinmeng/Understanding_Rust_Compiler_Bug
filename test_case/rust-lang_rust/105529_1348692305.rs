bash
cargo bisect-rustc --access=github --start 1.65.0 --end 2022-12-12 --regress=non-ice -- check
