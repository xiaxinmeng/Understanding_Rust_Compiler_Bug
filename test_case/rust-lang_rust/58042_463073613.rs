rust
#![feature(rustc_private)] // for libc, you can also use crates.io libc instead.

extern crate libc;

#[no_mangle]
pub unsafe extern "C" fn pthread_cond_timedwait(
    _cond: *mut libc::pthread_cond_t,
    _mutex: *mut libc::pthread_mutex_t,
    _abstime: *const libc::timespec
) -> libc::c_int {
    *libc::__errno_location() = libc::EINTR;
    return 1;
}

fn main() {
    let (s, r) = std::sync::mpsc::channel();
    s.send(()).unwrap();
    r.recv().unwrap();
    s.send(()).unwrap();
    r.recv().unwrap();
    r.recv_timeout(std::time::Duration::from_millis(1)).unwrap()
}
