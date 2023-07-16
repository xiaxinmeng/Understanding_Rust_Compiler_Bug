rust
$ ./stage1/bin/rustc file2.rs
error[E0072]: recursive type `Foo` has infinite size
 --> file2.rs:3:9
  |
3 |         struct $n {
  |         ^^^^^^^^^ recursive type has infinite size
...
9 | mk_struct!(Foo);
  | ---------------- in this macro invocation
