
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:4:5
  |
4 |     g2(|_: (), _: ()| {});
  |     ^^ -------------- found signature of `fn((), ()) -> _`
  |     |
  |     expected signature of `for<'r> fn(&'r (), for<'s> fn(&'s ())) -> _`
  |
note: required by `g2`
 --> src/main.rs:1:1
  |
1 | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
