bash
cargo bisect-rustc --access=github --regress=ice --start 1.63.0 --end 1.64.0 -- check
