
   Compiling rustc-main v0.0.0 (file:///C:/projects/rust/src/rustc)
error: unused attribute
  --> rustc\rustc.rs:16:47
   |
16 | #[cfg_attr(all(windows, target_env = "msvc"), link_args = "/STACK:16777216")]
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-attributes` implied by `-D warnings`
error: aborting due to previous error
