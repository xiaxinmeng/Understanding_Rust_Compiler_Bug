bash
$ time cargo test -p common --release

...
error: could not compile `messages`.

Caused by:
  process didn't exit successfully: `rustc --crate-name messages --edition=2018 messages/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C metadata=0c091483b2a87fc1 -C extra-filename=-0c091483b2a87fc1 --out-dir /tmp/cargo-target/release/deps -L dependency=/tmp/cargo-target/release/deps --extern asn1rs=/tmp/cargo-target/release/deps/libasn1rs-7f0d3cb210b4f673.rmeta --extern futures=/tmp/cargo-target/release/deps/libfutures-3a0be1c37a28e9a8.rmeta --extern serde=/tmp/cargo-target/release/deps/libserde-d9d8359d95432fa4.rmeta --extern serde_derive=/tmp/cargo-target/release/deps/libserde_derive-26bde9a45b227be1.so --extern tokio=/tmp/cargo-target/release/deps/libtokio-c76c350ce32195b9.rmeta --extern tokio_postgres=/tmp/cargo-target/release/deps/libtokio_postgres-e63ce98e1edc8138.rmeta` (signal: 9, SIGKILL: kill)

real    161m7,663s
user    169m13,774s
sys     0m35,055s
