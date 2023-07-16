
$ cargo rustc -vv --lib --release --target wasm32-unknown-unknown -- -C passes='deadargelim globaldce'
