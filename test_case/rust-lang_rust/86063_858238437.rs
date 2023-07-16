bash
cargo bisect-rustc --start=2021-04-01 --end 2021-06-05 --with-src -- run -Z build-std=core,alloc -v 
