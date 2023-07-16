
$ cat test.rs
fn f() { /* not in span */ }
fn main() {}
$ rustc test.rs
warning: function is never used: `f`
 --> test.rs:1:1
  |
1 | fn f() { /* not in span */ }
  | ^^^^^^
  |
  = note: #[warn(dead_code)] on by default
