bash
cargo bisect-rustc --end=2021-08-10 --regress=ice --access=github --without-cargo -- check
