rust
$ rustc file2.rs
error[E0072]: recursive type `Foo` has infinite size
 --> file2.rs:3:9
  |
3 |           struct $n {
  |  _________^ starting here...
4 | |             field: $n
5 | |         }
  | |_________^ ...ending here: recursive type has infinite size
...
9 |   mk_struct!(Foo);
  |   ---------------- in this macro invocation
