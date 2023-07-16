
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/perf-event-open-sys-0.3.1/src/lib.rs:103:25
    |
103 |         libc::ioctl(fd, ioctl as c_ulong, arg)
    |                         ^^^^^^^^^^^^^^^^ expected `i32`, found `u64`
    |
help: you can convert an `u64` to `i32` and panic if the converted value wouldn't fit
    |
103 |         libc::ioctl(fd, (ioctl as c_ulong).try_into().unwrap(), arg)
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
