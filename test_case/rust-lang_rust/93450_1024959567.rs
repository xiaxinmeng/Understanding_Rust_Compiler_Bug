bash
cargo bisect-rustc --preserve --start=2021-10-17 --end=2021-11-28 --script=./test.sh --regress=success 
