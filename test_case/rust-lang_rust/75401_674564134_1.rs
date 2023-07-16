rust
let errc = libc::sysctlbyname(
    lit_cstr!("hw.cpufrequency"),
    &mut out as *mut _
    &mut msize,
    null_mut(),
    0,
);
