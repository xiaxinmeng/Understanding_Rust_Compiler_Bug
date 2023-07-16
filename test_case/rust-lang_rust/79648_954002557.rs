
error[E0308]: mismatched types
  --> src/main.rs:20:5
   |
9  |             .filter_map(|_| async move { Some(()) })
   |                                        ------------
   |                                        |
   |                                        the expected `async` block
   |                                        the found `async` block
...
20 |     demand_is_send(get_foo())
   |     ^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected opaque type `impl futures::Future`
              found opaque type `impl futures::Future`
note: the lifetime requirement is introduced here
  --> src/main.rs:17:14
   |
17 |     where T: Send
   |              ^^^^
