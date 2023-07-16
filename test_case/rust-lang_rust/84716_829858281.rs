plain
[RUSTC-TIMING] addr2line test:false 0.413
[RUSTC-TIMING] core test:false 37.795
[RUSTC-TIMING] gimli test:false 5.025
[RUSTC-TIMING] object test:false 9.979
error[E0425]: cannot find function `chroot` in crate `libc`
     |
     |
1334 |     cvt(unsafe { libc::chroot(dir.as_ptr()) })?;
     |                        ^^^^^^ not found in `libc`
help: consider importing this function
     |
1    | use crate::sys::ext::fs::chroot;
     |
