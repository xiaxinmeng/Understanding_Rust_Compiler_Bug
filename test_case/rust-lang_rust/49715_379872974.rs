
   Compiling rustdoc-tool v0.0.0 (file:///C:/projects/rust/src/tools/rustdoc)
error: unused attribute
  --> tools\rustdoc\main.rs:14:47
   |
14 | #[cfg_attr(all(windows, target_env = "msvc"), link_args = "/STACK:16777216")]
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-attributes` implied by `-D warnings`
error: aborting due to previous error
