 rs
    pub fn inner_read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        let ret = retry(|| unsafe {
            libc::read(self.fd(),
                       buf.as_mut_ptr() as *mut libc::c_void,
                       buf.len() as libc::size_t) as libc::c_int
        });
        if ret == 0 {
            Err(util::eof())
        } else if ret < 0 {
            Err(super::last_error())
        } else {
            Ok(ret as uint)
        }
    }
