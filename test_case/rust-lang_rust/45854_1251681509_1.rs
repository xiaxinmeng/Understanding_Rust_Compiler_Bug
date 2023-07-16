
   Compiling rustdoc v0.0.0 (/builddir/build/BUILD/rustc-1.64.0-src/src/librustdoc)
error[E0786]: found invalid metadata files for crate `rustc_monomorphize` which `rustc_driver` depends on
  --> src/librustdoc/lib.rs:41:1
   |
41 | extern crate rustc_driver;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: failed to mmap file '/builddir/build/BUILD/rustc-1.64.0-src/build/armv7-unknown-linux-gnueabihf/stage1/lib/rustlib/armv7-unknown-linux-gnueabihf/lib/librustc_monomorphize-5bd38fc25e73bfbd.rlib': Cannot allocate memory (os error 12)
error[E0786]: found invalid metadata files for crate `rustc_monomorphize` which `rustc_interface` depends on
  --> src/librustdoc/lib.rs:49:1
   |
49 | extern crate rustc_interface;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: failed to mmap file '/builddir/build/BUILD/rustc-1.64.0-src/build/armv7-unknown-linux-gnueabihf/stage1/lib/rustlib/armv7-unknown-linux-gnueabihf/lib/librustc_monomorphize-5bd38fc25e73bfbd.rlib': Cannot allocate memory (os error 12)
error[E0786]: found invalid metadata files for crate `askama_derive` which `askama` depends on
  --> src/librustdoc/html/layout.rs:10:5
   |
10 | use askama::Template;
   |     ^^^^^^
   |
   = note: failed to mmap file '/builddir/build/BUILD/rustc-1.64.0-src/build/armv7-unknown-linux-gnueabihf/stage1-tools/release/deps/libaskama_derive-20ded99bdf04465e.so': Cannot allocate memory (os error 12)
error[E0786]: found invalid metadata files for crate `askama_derive` which `askama` depends on
  --> src/librustdoc/html/render/print_item.rs:38:5
   |
38 | use askama::Template;
   |     ^^^^^^
   |
   = note: failed to mmap file '/builddir/build/BUILD/rustc-1.64.0-src/build/armv7-unknown-linux-gnueabihf/stage1-tools/release/deps/libaskama_derive-20ded99bdf04465e.so': Cannot allocate memory (os error 12)
error: cannot determine resolution for the derive macro `Template`
  --> src/librustdoc/html/layout.rs:42:10
   |
42 | #[derive(Template)]
   |          ^^^^^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot find attribute `template` in this scope
  --> src/librustdoc/html/layout.rs:43:3
   |
43 | #[template(path = "page.html")]
   |   ^^^^^^^^
error: cannot determine resolution for the derive macro `Template`
  --> src/librustdoc/html/render/print_item.rs:52:10
   |
52 | #[derive(Template)]
   |          ^^^^^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot find attribute `template` in this scope
  --> src/librustdoc/html/render/print_item.rs:53:3
   |
53 | #[template(path = "print_item.html")]
   |   ^^^^^^^^
memory allocation of 2097152 bytes failed
rustc exited with signal: 6 (SIGABRT) (core dumped)
error: could not compile `rustdoc` due to 8 previous errors
