
error: higher-ranked lifetime error
 --> src/lib.rs:9:5
  |
9 |     Box::new(async { new(|| async { f().await }).await })
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: could not prove `for<'r, 's> Box<impl for<'r> Future<Output = ()>>: CoerceUnsized<Box<(dyn Future<Output = ()> + Send + 's)>>`
