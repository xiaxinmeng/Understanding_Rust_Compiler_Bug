
fn foo<F>(_m: F) where F: Fn(usize) {}
----------
error[E0281]: type mismatch: `[closure@src/lib.rs:11:13: 11:26]` implements the trait `core::ops::FnOnce<(isize,)>`, but the trait `core::ops::FnOnce<(usize,)>` is required
  --> src/lib.rs:11:9
   |
11 |         foo(|y: isize| ());
   |         ^^^ ------------- implements `core::ops::FnOnce<(isize,)>`
   |         |
   |         requires `core::ops::FnOnce<(usize,)>`
   |         expected usize, found isize
