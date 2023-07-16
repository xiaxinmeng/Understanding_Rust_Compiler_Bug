
pub struct RWLock { 
    inner: UnsafeCell<c::SRWLOCK>,
    thread_local_reading: DWORD,
}

impl RWLock {
    pub /*const*/ fn new() -> RWLock { // cannot const anymore
        RWLock { 
           inner: UnsafeCell::new(c::SRWLOCK_INIT),
           thread_local_reading: c::TlsAlloc(), // free in drop!
        }
    }
    #[inline]
    pub unsafe fn read(&self) {
        if c::TlsGetValue(self.thread_local_reading) != 0 { panic!("recursive read") }
        TlsSetValue(self.thread_local_reading, 1);
        c::AcquireSRWLockShared(self.inner.get())
    }
    /* similar for try_read */

    #[inline]
    pub unsafe fn read_unlock(&self) {
        TlsSetValue(self.thread_local_reading, 0);
        c::ReleaseSRWLockShared(self.inner.get())
    }
}
