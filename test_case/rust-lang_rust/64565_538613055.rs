
error[E0277]: `impl Sync` cannot be sent between threads safely
 --> src/lib.rs:4:13
  |
1 | fn is_send<T: Send>(val: T) {}
  |    -------    ---- required by this bound in `is_send`
...
4 |     is_send(val);
  |             ^^^ `impl Sync` cannot be sent between threads safely
  |
  = help: the trait `std::marker::Send` is not implemented for `impl Sync`
  = help: consider adding a `where impl Sync: std::marker::Send` bound
