sh
cargo-bisect-rustc -v --preserve --target wasm32-unknown-unknown --start 2020-04-08 --end 2020-12-31 --script `pwd`/issue-80703.sh
