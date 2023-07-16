

error[E0309]: the parameter type `T` may not live long enough
  --> src/lib.rs:14:21
   |
14 |     type Item<'a> = &'a T;
   |                     ^^^^^- help: consider adding a where clause: `where T: 'a`
   |                     |
   |                     ...so that the reference type `&'a T` does not outlive the data it points at

For more information about this error, try `rustc --explain E0309`.
error: could not compile `playground` due to previous error
