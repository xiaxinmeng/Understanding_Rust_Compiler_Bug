
   Compiling proc_macro_plugin v0.0.0 (file:///Users/ikerins/rust/rust/src/libproc_macro_plugin)
error[E0523]: found two different crates with name `rustc_llvm` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
  --> src/libproc_macro_plugin/lib.rs:89:1
   |
89 | extern crate rustc_plugin;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `proc_macro_plugin`.
