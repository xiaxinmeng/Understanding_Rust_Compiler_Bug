
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:4:5
  |
4 |     g2(|_: (), _: ()| {});
  |     ^^ --------------
  |     |  |   |
  |     |  |   help: consider borrowing the argument: `&()`
  |     |  found signature defined here
  |     expected due to this
  |
  = note: expected closure signature `for<'a> fn(&'a (), for<'a> fn(&'a ())) -> _`
             found closure signature `fn((), ()) -> _`
note: required by a bound in `g2`
 --> src/main.rs:1:25
  |
1 | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
  |                         ^^^^^^^^^^^^^^^^ required by this bound in `g2`
