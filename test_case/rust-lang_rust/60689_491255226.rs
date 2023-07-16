rust
extern crate libc;
use std::os::unix::io::AsRawFd;
use std::{io, ptr};


// mutability is not technically required but it fits API conventions
pub fn send_file<R: AsRawFd, W: AsRawFd>(r: &mut R, w: &mut W) -> io::Result<usize> {
    // parameter ordering is reversed
    // null pointer is an out-pointer for the offset after the read, if not null it doesn't update the file cursors which we actually do want
    // last argument is the maximum number of bytes to copy but 
    // the documentation says it stops at 2^31-1 regardless of arch
    match unsafe { libc::sendfile(w.as_raw_fd(), r.as_raw_fd(), ptr::null_mut(), usize::MAX) } {
        -1 => Err(io::Error::last_os_error()),
        copied => Ok(copied as usize), 
    } 
}
