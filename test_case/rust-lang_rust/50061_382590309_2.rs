
$ rustc +nightly foo.rs
$ rustc +nightly bar.rs -L .
error[E0425]: cannot find value `s` in this scope
 --> bar.rs:9:17
  |
9 |         let _ = $v;
  |                 ^^ not found in this scope

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
