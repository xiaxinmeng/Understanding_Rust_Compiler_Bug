
error[E0478]: lifetime bound not satisfied
 --> src/lib.rs:9:20
  |
9 |     type Fut<'a> = impl ::std::future::Future + 'a
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: lifetime parameter instantiated with the empty lifetime
note: but lifetime parameter must outlive the lifetime `'a` as defined here
 --> src/lib.rs:9:14
  |
9 |     type Fut<'a> = impl ::std::future::Future + 'a
  |              ^^

For more information about this error, try `rustc --explain E0478`.

