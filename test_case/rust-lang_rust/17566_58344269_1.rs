 rust
    fn drop(&mut self) {
        drop(unsafe { replace(self.ptr, uninitialized()) });
    }
