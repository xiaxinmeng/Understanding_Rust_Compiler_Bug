
error[E0460]: found possibly newer version of crate `std` which `rand` depends on                                                                    [25/1841]
  --> libstd/tests/env.rs:11:1
   |
11 | extern crate rand;
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/corentih/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-a5fba866ec21af50.rlib
           crate `std`: /home/corentih/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd-a5fba866ec21af50.rlib
           crate `std`: /home/corentih/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd-a5fba866ec21af50.so
           crate `std`: /home/corentih/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-a5fba866ec21af50.so
           crate `rand`: /home/corentih/rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand-f4e190f4f182b613.rlib

error: aborting due to previous error
