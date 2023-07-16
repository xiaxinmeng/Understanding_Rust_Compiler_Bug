 rust
use sync::atomic::{AtomicBool, Ordering};

pub struct RWLock { 
    inner: UnsafeCell<c::SRWLOCK> ,
    downgrade_flag: AtomicBool,
}

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock { 
            inner: UnsafeCell::new(c::SRWLOCK_INIT) ,
            downgrade_flag: AtomicBool::new(false),
        }
    }

    // ...

    #[inline]
    pub unsafe fn write(&self) {
        // loop until we both acquire the lock and the downgrade flag is not set.
        loop {
            c::AcquireSRWLockExclusive(self.inner.get())
            if !self.downgrade_flag.load(Ordering::SeqCst) {
                return;
            }

            self.write_unlock();
        }
    }
    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        // if we manage to acquire the write lock but the downgrade flag is set, unlock and
        // return. could also do a loop here, though.
        if c::TryAcquireSRWLockExclusive(self.inner.get()) != 0 {
            if !self.downgrade_flag.load(Ordering::SeqCst) {
                return true;
            }
            self.write_unlock();
        }
        false
    }

    #[inline]
    pub unsafe fn downgrade(&self) {
        self.downgrade_flag.store(true, Ordering::SeqCst);
        self.write_unlock();
        self.read();
        self.downgrade_flag.store(false, Ordering::SeqCst);
    }

    // ...
}
