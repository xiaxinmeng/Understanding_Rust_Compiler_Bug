
[misdreavus@tonberry libsecp256k1]$ rustdoc +nightly-2018-01-29 --version
rustdoc 1.25.0-nightly (21882aad7 2018-01-28)
[misdreavus@tonberry libsecp256k1]$ cargo +nightly-2018-01-29 doc
 Documenting libsecp256k1 v0.1.12 (file:///home/misdreavus/clones/libsecp256k1)
thread 'rustc' panicked at 'librustc/lint/context.rs:1049: failed to process buffered lint here', librustc/session/mod.rs:1170:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (21882aad7 2018-01-28) running on x86_64-unknown-linux-gnu

error: Could not document `libsecp256k1`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name secp256k1 src/lib.rs -o /home/misdreavus/clones/libsecp256k1/target/doc -L dependency=/home/misdreavus/clones/libsecp256k1/target/debug/deps --extern hmac_drbg=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libhmac_drbg-52f8ac033526d945.rlib --extern sha2=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libsha2-fcbda550b505ab8d.rlib --extern typenum=/home/misdreavus/clones/libsecp256k1/target/debug/deps/libtypenum-0209d654c67b5a67.rlib --extern rand=/home/misdreavus/clones/libsecp256k1/target/debug/deps/librand-3c5287050d3409be.rlib` (exit code: 101)
