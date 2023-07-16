rust

        let len = self.len;
        if len == self.buf.cap() {
            self.buf.double();
        }
        unsafe {
            let end = self.as_mut_ptr().offset(len as isize);
            ptr::write(end, value);
            self.len = len + 1;
        }

