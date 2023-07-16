
error[E0308]: mismatched types
   --> library/std/src/sys/unix/time.rs:293:39
    |
293 |             const clock_id: clock_t = libc::CLOCK_UPTIME_RAW;
    |                                       ^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`

error[E0308]: mismatched types
   --> library/std/src/sys/unix/time.rs:323:43
    |
323 |             SystemTime { t: Timespec::now(libc::CLOCK_REALTIME) }
    |                             ------------- ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
    |                             |
    |                             arguments to this function are incorrect
    |
note: associated function defined here
   --> library/std/src/sys/unix/time.rs:333:16
    |
333 |         pub fn now(clock: clock_t) -> Timespec {
    |                ^^^ --------------
help: you can convert a `u32` to an `i32` and panic if the converted value doesn't fit
    |
323 |             SystemTime { t: Timespec::now(libc::CLOCK_REALTIME.try_into().unwrap()) }
    |                                                               ++++++++++++++++++++

error[E0308]: mismatched types
    --> library/std/src/sys/unix/time.rs:362:46
     |
362  |             cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
     |                          ------------------- ^^^^^ expected `u32`, found `i32`
     |                          |
     |                          arguments to this function are incorrect
     |
note: function defined here
    --> /Users/thom/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.135/src/unix/bsd/apple/mod.rs:5092:12
     |
5092 |     pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
     |            ^^^^^^^^^^^^^
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
     |
362  |             cvt(unsafe { libc::clock_gettime(clock.try_into().unwrap(), t.as_mut_ptr()) }).unwrap();
     |                                                   ++++++++++++++++++++
