
$ cargo +nightly --version
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src\libcore\option.rs:323
note: Run with `RUST_BACKTRACE=1` for a backtrace.

$ cargo +nightly-2017-03-16 --version
cargo 0.18.0-nightly (4a3c0a63b 2017-03-12)
