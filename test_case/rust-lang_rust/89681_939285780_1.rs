
error[E0277]: the trait bound `[closure@src/lib.rs:27:19: 27:24]: Handler<_>` is not satisfied
  --> src/lib.rs:27:5
   |
21 |     fn check_handler<H, T>(_: H)
   |        ------------- required by a bound in this
22 |     where
23 |         H: Handler<T>,
   |            ---------- required by this bound in `check_handler`
...
27 |     check_handler(|| {});
   |     ^^^^^^^^^^^^^ the trait `Handler<_>` is not implemented for `[closure@src/lib.rs:27:19: 27:24]`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bad-error` due to previous error
