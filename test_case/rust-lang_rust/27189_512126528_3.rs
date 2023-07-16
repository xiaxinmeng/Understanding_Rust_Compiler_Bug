
error: argument never used
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:7:9
  |
3 |         "Hello, world! {}",
  |         ------------------ formatting specifier missing
...
7 |         "arg when bad",
  |         ^^^^^^^^^^^^^^ argument never used

error[E0658]: attributes on expressions are experimental
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:4:9
  |
4 |         #[cfg(not(feature = "bad"))]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/15701
  = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

error: removing an expression is not supported in this position
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:4:9
  |
4 |         #[cfg(not(feature = "bad"))]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0658]: attributes on expressions are experimental
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:6:9
  |
6 |         #[cfg(feature = "bad")]
  |         ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/15701
  = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

error: removing an expression is not supported in this position
 --> /home/user/build/2nonpkgs/rust.stuff/plaincatchup/bad_Z/src/main.rs:6:9
  |
6 |         #[cfg(feature = "bad")]
  |         ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
error: Could not compile `bad_Z`.
