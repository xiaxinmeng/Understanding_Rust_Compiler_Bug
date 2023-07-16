
[RUSTC-TIMING] addr2line test:false 0.596
[RUSTC-TIMING] core test:false 27.851
[RUSTC-TIMING] gimli test:false 6.055
[RUSTC-TIMING] object test:false 6.197
error[E0425]: cannot find value `MAP_STACK` in crate `libc`
   --> library/std/src/sys/unix/stack_overflow.rs:150:52
    |
150 |         let flags = MAP_PRIVATE | MAP_ANON | libc::MAP_STACK;
    |                                                    ^^^^^^^^^ not found in `libc`
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 2.972
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:06:57
