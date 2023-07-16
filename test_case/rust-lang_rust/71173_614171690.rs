
error[E0464]: multiple matching crates for `getopts`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_session-651.0.0/lib.rs:6:1
  |
6 | extern crate getopts;
  | ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: candidates:
          crate `getopts`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-e81d628ddb13670c.rlib
          crate `getopts`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-1b0bb056d1f90b77.rlib

error[E0463]: can't find crate for `getopts`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_session-651.0.0/lib.rs:6:1
  |
6 | extern crate getopts;
  | ^^^^^^^^^^^^^^^^^^^^^ can't find crate
