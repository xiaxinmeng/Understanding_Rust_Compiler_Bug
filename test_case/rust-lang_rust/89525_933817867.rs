plain

error[E0432]: unresolved import `crate::sys::cvt`
  --> library/std/src/os/fd/owned.rs:11:5
   |
11 | use crate::sys::cvt;
   |     ^^^^^^^^^^^^^^^ no `cvt` in `sys`

error[E0425]: cannot find value `F_DUPFD_CLOEXEC` in crate `libc`
    |
    |
79  |         let cmd = libc::F_DUPFD_CLOEXEC;
    |                         ^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `FD_CLOEXEC`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.99/src/wasi.rs:188:1
    |
    |
188 | pub const FD_CLOEXEC: c_int = 1;
    | -------------------------------- similarly named constant `FD_CLOEXEC` defined here
Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 2.389
warning: `std` (lib) generated 1 warning
