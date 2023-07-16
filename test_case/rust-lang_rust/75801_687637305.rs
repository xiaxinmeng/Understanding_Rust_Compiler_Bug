
error[E0308]: mismatched types
 --> src/main.rs:9:6
  |
5 |             let _bar: u32 = $arg;
  |                       --- expected due to this
...
9 | foo!("baz");
  |      ^^^^^ expected `u32`, found `&str`

error: aborting due to previous error
