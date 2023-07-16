bash
cargo bisect-rustc 2020-07-01 --end 2020-08-01 --preserve --regress=ice --without-cargo -- check 
