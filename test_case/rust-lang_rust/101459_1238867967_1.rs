
Compiling playground v0.0.1 (/playground)
error[[E0382]](https://doc.rust-lang.org/stable/error-index.html#E0382): use of moved value: `a`
 --> src/main.rs:8:21
  |
6 |     let a = String::from("xyz");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
7 |     takes_ownership(a);
  |                     - value moved here
8 |     takes_ownership(a);
  |                     ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground` due to previous error
