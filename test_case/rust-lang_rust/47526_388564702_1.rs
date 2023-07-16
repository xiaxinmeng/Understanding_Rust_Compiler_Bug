
$ RUSTFLAGS="-C passes='deadargelim globaldce'" cargo build -vv --release --target wasm32-unknown-unknown
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -C passes='deadargelim globaldce' --target wasm32-unknown-unknown --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro` (exit code: 101)
--- stderr
error: multiple input filenames provided
