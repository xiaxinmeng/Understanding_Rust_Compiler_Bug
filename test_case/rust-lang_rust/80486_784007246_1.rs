rust
    if let Ok(_) = DRAIN_MODE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
