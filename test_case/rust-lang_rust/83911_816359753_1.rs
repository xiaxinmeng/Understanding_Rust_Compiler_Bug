bash
cargo bisect-rustc --start 2021-03-31 --end 2021-04-02 --regress=non-error -- build --release 
