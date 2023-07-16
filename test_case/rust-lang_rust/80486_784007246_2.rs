rust
    if DRAIN_MODE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
