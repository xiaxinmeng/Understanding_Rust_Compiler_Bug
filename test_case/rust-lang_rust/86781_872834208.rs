bash
cargo bisect-rustc --end=2021-06-01 --access=github --regress=ice --without-cargo -- check
