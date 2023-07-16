
$ git clone git@github.com:acmcarther/cucumber && cd cucumber
$ rustup toolchain install beta-2018-04-06-x86_64-unknown-linux-gnu 
$ rustup default beta-2018-04-06-x86_64-unknown-linux-gnu
$ CARGO_INCREMENTAL=0 RUST_BACKTRACE=full RUSTFLAGS=--cap-lints=warn cargo test --no-run --frozen
# Build succeeds
