 rust
    #[inline]
    pub unsafe fn read(&self) {
        let r = ffi::pthread_rwlock_rdlock(self.inner.get());
        debug_assert_eq!(r, 0);
    }
