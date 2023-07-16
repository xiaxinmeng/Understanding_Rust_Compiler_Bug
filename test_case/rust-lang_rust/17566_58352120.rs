 rust
    fn drop(&mut self) {
        unsafe {
            ptr::read(self.ptr); // could be wrapped in drop(...)
        }
    }
