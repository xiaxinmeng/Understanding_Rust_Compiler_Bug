plain
[RUSTC-TIMING] core test:false 24.706
[RUSTC-TIMING] addr2line test:false 0.529
[RUSTC-TIMING] gimli test:false 5.858
[RUSTC-TIMING] object test:false 5.890
error[E0531]: cannot find unit struct, unit variant or constant `SIGSTKFLT` in crate `libc`
    |
    |
736 |         libc::SIGSTKFLT => " (SIGSTKFLT)",
    |               ^^^^^^^^^ help: a constant with a similar name exists: `SIGSTKSZ`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.126/src/unix/linux_like/linux/gnu/b64/mips64/mod.rs:817:1
    |
    |
817 | pub const SIGSTKSZ: ::size_t = 8192;
    | ------------------------------------ similarly named constant `SIGSTKSZ` defined here
For more information about this error, try `rustc --explain E0531`.
[RUSTC-TIMING] std test:false 2.995
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:06:40
