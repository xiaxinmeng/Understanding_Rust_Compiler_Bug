 rust
pub fn current_id() -> ThreadId {
    ThreadId {
        thread: libc::pthread_self(),
    }
}
