
rustc +nightly-2020-09-14 a.rs -Zmir-opt-level=2 && ./a
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', a.rs:9:5
