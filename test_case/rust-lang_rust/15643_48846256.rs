 Rust
    pub fn fd(&self) -> fd_t { self.inner.fd }

    pub fn handle(&self) -> libc::HANDLE {
        unsafe { libc::get_osfhandle(self.fd()) as libc::HANDLE }
    }
