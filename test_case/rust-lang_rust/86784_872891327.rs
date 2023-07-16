bash
cargo bisect-rustc --end=2021-07-01 --access=github --regress=error --without-cargo -- build
