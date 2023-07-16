
error[E0786]: found invalid metadata files for crate `rustc_driver`
  --> src/librustdoc/lib.rs:41:1
   |
41 | extern crate rustc_driver;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: failed to mmap file '/builddir/build/BUILD/rustc-1.64.0-src/build/i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_driver-995e686975e44b6b.so': Cannot allocate memory (os error 12)
For more information about this error, try `rustc --explain E0786`.
error: could not compile `rustdoc` due to previous error
