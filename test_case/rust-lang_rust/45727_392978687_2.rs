
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:4:5
  |
1 | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
  |    --                   ---------------- required by this bound in `g2`
...
4 |     g2(|_: (), _: ()| {});
  |     ^^ -------------- found signature of `fn((), ()) -> _`
  |     |
  |     expected signature of `for<'r> fn(&'r (), for<'s> fn(&'s ())) -> _`
