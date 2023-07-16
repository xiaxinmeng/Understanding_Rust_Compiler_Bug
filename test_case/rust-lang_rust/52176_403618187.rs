
error[E0460]: found possibly newer version of crate `std` which `rand` depends on
  --> libcore/../libcore/tests/lib.rs:50:1
   |
50 | extern crate rand;
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acba8e5dff5ff58f.rlib
           crate `std`: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-acba8e5dff5ff58f.rlib
           crate `std`: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-acba8e5dff5ff58f.so
           crate `std`: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-acba8e5dff5ff58f.so
           crate `rand`: /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librand-467b6a656af26977.rlib
