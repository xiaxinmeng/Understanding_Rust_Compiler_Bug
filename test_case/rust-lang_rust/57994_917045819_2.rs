
error[E0599]: no method named `poll` found for opaque type `impl Future` in the current scope
  --> src/lib.rs:8:9
   |
8  |     f().poll(cx);
   |         ^^^^ method not found in `impl Future`
   |
help: consider wrapping the receiver expression with the appropriate type
   |
8  |     Pin::new(&mut f()).poll(cx);
   |     +++++++++++++    +
