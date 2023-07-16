bash
cargo bisect-rustc --access=github --start 2022-11-18 --end 2022-11-20 --regress=ice -- build
