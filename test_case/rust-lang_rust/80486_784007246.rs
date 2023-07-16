rust
    if !DRAIN_MODE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).unwrap_or_else(|x| x) {
