plain
[RUSTC-TIMING] addr2line test:false 0.567
[RUSTC-TIMING] core test:false 19.535
[RUSTC-TIMING] gimli test:false 5.321
[RUSTC-TIMING] object test:false 5.463
error[E0425]: cannot find function, tuple struct or tuple variant `CPU_COUNT` in crate `libc`
    |
    |
282 |                     let count = unsafe { libc::CPU_COUNT(&set) };
    |                                                ^^^^^^^^^ not found in `libc`
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 2.715
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:13:32
