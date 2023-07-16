rust
#[no_mangle]
pub extern "C" fn getpid() -> pid_t {
    // Do wrapper stuff...
    next!(getpid())
}

#[no_mangle]
pub extern "C" fn recv(fd: c_int, buf: *mut c_void, n: usize, flags: c_int) -> isize {
    // Do wrapper stuff..
    next!(recv(fd, buf, n, flags))
}
