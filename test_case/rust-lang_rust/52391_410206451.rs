rust
unsafe fn read_atomic_relaxed(ptr: *const u8) -> u8 {
    use std::sync::atomic;
    let aptr = ptr as *const atomic::AtomicU8;
    (&*aptr).load(atomic::Ordering::Relaxed)
}
