 rust
pub fn duplicate(&self) -> io::Result<Socket> {
    cvt(unsafe {
        libc::fcntl(self.0.raw(), libc::FCNTL_DUPFD_CLOEXEC, 0)
    }).or_else(|e| {
        if e.raw_os_error() == Some(libc::EINVAL) {
            cvt(unsafe { libc::fcntl(self.0.raw(), libc::FCNTL_DUPFD, 0) })
        } else {
            Err(e)
        }
    }).map(|fd| {
        let fd = FileDesc::new();
        fd.set_cloexec();
        Socket(fd)
    })
}
