
 > ../rust/x.py build |& tee /tmp/rust.log
Updating submodules
   Compiling bootstrap v0.0.0 (file:///work/rust/rust/src/bootstrap)
error: Shared object "libproc_macro-33f3fe1937fc08f8.so" not found
   --> /work/rust/rust/src/bootstrap/lib.rs:123:1
    |
123 | extern crate serde_derive;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `bootstrap`.
