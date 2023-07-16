rust
    if !DRAIN_MODE.compare_and_swap(false, true, Ordering::Relaxed) {
