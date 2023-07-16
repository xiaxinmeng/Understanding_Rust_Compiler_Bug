
error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
 --> src/x.rs:8:18
  |
8 |         Ok::<(), impl std::error::Error>(())
  |                  ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
