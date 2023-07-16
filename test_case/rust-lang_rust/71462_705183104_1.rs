
error[E0277]: `<Inner as Service<&'a ()>>::Future` cannot be sent between threads safely
  --> src/main.rs:29:5
   |
23 | fn is_service<S>(_: S)
   |    ---------- required by a bound in this
24 | where
25 |     S: Service<()>
   |        ----------- required by this bound in `is_service`
...
29 |     is_service(Outer { inner: Inner })
   |     ^^^^^^^^^^ `<Inner as Service<&'a ()>>::Future` cannot be sent between threads safely
   |
   = help: the trait `for<'a> std::marker::Send` is not implemented for `<Inner as Service<&'a ()>>::Future`
   = note: required because of the requirements on the impl of `Service<()>` for `Outer<Inner>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
