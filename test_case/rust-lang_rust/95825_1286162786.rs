shell
$ rustc --version
rustc 1.61.0-nightly (03918badd 2022-03-07)
$ RUSTFLAGS="-C instrument-coverage" cargo test --all-features
...
   Doc-tests lib
error[E0463]: can't find crate for `serde_derive` which `serde` depends on
 --> /home/dup2rt/Temp/rust-bug-95825/lib/src/cbor_types.rs:1:5
  |
1 | use serde::de::{Deserialize, Deserializer};
  |     ^^^^^ can't find crate
...
