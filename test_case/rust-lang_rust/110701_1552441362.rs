
error[E0460]: found possibly newer version of crate `std` which `thin_vec` depends on
  --> src/librustdoc/lib.rs:24:1
   |
24 | extern crate thin_vec;
   | ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/jyn/src/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-9768f62f7b152226.rlib
           crate `std`: /home/jyn/src/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-9768f62f7b152226.so
           crate `thin_vec`: /home/jyn/src/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libthin_vec-812d4d34d6de684d.rmeta
