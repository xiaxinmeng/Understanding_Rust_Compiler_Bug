
[misdreavus@tonberry libsecp256k1]$ cargo +nightly --version
cargo 0.26.0-nightly (1d6dfea44 2018-01-26)
[misdreavus@tonberry libsecp256k1]$ cargo +nightly doc
 Documenting libsecp256k1 v0.1.12 (file:///home/misdreavus/clones/libsecp256k1)
thread 'rustc' panicked at 'librustc/lint/context.rs:1049: failed to process buffered lint here', librustc/session/mod.rs:1173:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (8ccab7eed 2018-01-31) running on x86_64-unknown-linux-gnu

error: Could not document `libsecp256k1`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name secp256k1 src/lib.rs -o /home/misdreavus/clones/libsecp256k1/target/doc -L dependency=/home/misdreavus/clones/libsecp256k1/target/debug/deps --extern sha2=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libsha2-e4d13803b739e98a.rlib --extern hmac_drbg=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libhmac_drbg-51bff27146c63e09.rlib --extern typenum=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libtypenum-c5b2d1a53f449fee.rlib --extern rand=/home/misdreavus/clones/libsecp256k1/target/debug/deps/librand-f5a926d74242e0b8.rlib` (exit code: 101)
