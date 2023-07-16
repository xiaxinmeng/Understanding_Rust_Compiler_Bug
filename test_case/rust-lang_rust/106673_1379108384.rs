rust
fn futex_wait(addr: &AtomicU32, val: u32) {
    pthread_sleepon_lock();
    if addr.load(Relaxed) == val {
        pthread_sleepon_wait(addr.as_mut_ptr().cast());
    }
    pthread_sleepon_unlock();
}

fn futex_wake(addr: &AtomicU32) {
    pthread_sleepon_lock();
    pthread_sleepon_signal(addr.as_mut_ptr().cast());
    pthread_sleepon_unlock();
}
