
$ cargo +beta-2022-12-29 --version
cargo 1.67.0-beta.5 (f6e737b1e 2022-12-02)

$ CARGO_INCREMENTAL=0 RUSTFLAGS=--cap-lints=warn \
    cargo +beta-2022-12-29 test --no-run --frozen
