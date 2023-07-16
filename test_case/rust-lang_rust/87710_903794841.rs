plain
   Compiling hashbrown v0.11.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling object v0.26.1
   Compiling addr2line v0.16.0
error: unnecessary `unsafe` block
    |
    |
551 |     unsafe fn new(pid: pid_t, pidfd: pid_t) -> Self {
    |     ----------------------------------------------- because it's nested under this `unsafe` fn
...
556 |             .then(|| PidFd::from_inner(unsafe { sys::fd::FileDesc::from_raw_fd(pidfd) }));
    |                                        ^^^^^^ unnecessary `unsafe` block
    |
    = note: `-D unused-unsafe` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:04:52
