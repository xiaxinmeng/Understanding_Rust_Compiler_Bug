plain
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/sys/unix/time.rs:296:30
    |
296 |             Instant { t: now(libc::CLOCK_MONOTONIC) }
    |                              |
    |                              expected `i64`, found `i32`
    |                              expected `i64`, found `i32`
    |                              help: you can convert an `i32` to an `i64`: `libc::CLOCK_MONOTONIC.into()`
error[E0308]: mismatched types
   --> library/std/src/sys/unix/time.rs:333:33
    |
    |
333 |             SystemTime { t: now(libc::CLOCK_REALTIME) }
    |                                 |
    |                                 expected `i64`, found `i32`
    |                                 expected `i64`, found `i32`
    |                                 help: you can convert an `i32` to an `i64`: `libc::CLOCK_REALTIME.into()`
error[E0308]: mismatched types
   --> library/std/src/sys/unix/time.rs:371:42
    |
    |
371 |         cvt(unsafe { libc::clock_gettime(clock, &mut t.t) }).unwrap();
    |                                          ^^^^^ expected `i32`, found `i64`
    |
help: you can convert an `i64` to an `i32` and panic if the converted value doesn't fit
    |
371 |         cvt(unsafe { libc::clock_gettime(clock.try_into().unwrap(), &mut t.t) }).unwrap();

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:01:14
