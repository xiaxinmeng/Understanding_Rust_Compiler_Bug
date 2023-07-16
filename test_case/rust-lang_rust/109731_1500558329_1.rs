
$ rustc +nightly -V
rustc 1.70.0-nightly (28a29282f 2023-04-06)
$ rustc +nightly test.rs
error: this arithmetic operation will overflow
 --> test.rs:6:18
  |
6 |             dbg!(max + 1);
  |                  ^^^^^^^ attempt to compute `usize::MAX + 1_usize`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
