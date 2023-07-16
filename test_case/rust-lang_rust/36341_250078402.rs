 rust
pub struct ThreadId {
    thread: libc::pthread_t,
}

impl PartialEq for ThreadId {
    fn eq(&self, other: &ThreadId) -> bool {
        unsafe {
            libc::pthread_equal(self.thread, other.thread) != 0
        }
    }
}
