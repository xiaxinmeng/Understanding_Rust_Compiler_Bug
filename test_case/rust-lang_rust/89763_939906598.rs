plain
[RUSTC-TIMING] core test:false 15.649
[RUSTC-TIMING] addr2line test:false 0.427
[RUSTC-TIMING] gimli test:false 4.080
[RUSTC-TIMING] object test:false 4.120
error[E0425]: cannot find function `preadv` in crate `libc`
   --> library/std/src/sys/unix/fd.rs:133:19
133 |             libc::preadv(
    |                   ^^^^^^ help: a function with a similar name exists: `pread`
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.103/src/unix/mod.rs:933:5
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.103/src/unix/mod.rs:933:5
    |
933 |     pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
    |     ------------------------------------------------------------------------------------------- similarly named function `pread` defined here

error[E0425]: cannot find function `pwritev` in crate `libc`
   --> library/std/src/sys/unix/fd.rs:213:19
    |
213 |             libc::pwritev(
    |                   ^^^^^^^ help: a function with a similar name exists: `pwrite`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.103/src/unix/mod.rs:938:5
    |
    |
938 |     pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
    |     ---------------------------------------------------------------------------------------------- similarly named function `pwrite` defined here
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std test:false 1.946
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:15:29
