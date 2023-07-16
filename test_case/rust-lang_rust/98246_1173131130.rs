plain
   Compiling addr2line v0.16.0
[RUSTC-TIMING] addr2line test:false 0.551
[RUSTC-TIMING] gimli test:false 5.504
[RUSTC-TIMING] object test:false 5.810
error[E0425]: cannot find function `futimes` in crate `libc`
     |
     |
1078 | ...                   libc::futimes(self.as_raw_fd(), timevals.as_ptr())
     |                             ^^^^^^^ help: a function with a similar name exists: `futimens`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.126/src/unix/linux_like/mod.rs:1653:5
     |
     |
1653 |     pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
     |     ----------------------------------------------------------------- similarly named function `futimens` defined here
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 3.014
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:12:23
