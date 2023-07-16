rust
loop {
    if some_condition {
        return false;
    }
    match atomic.compare_exchange_weak(
        oldval,
        newval,
        Ordering::Relaxed,
        Ordering::Relaxed,
    ) {
        Ok(_) => return true,
        Err(x) => oldval = x,
    }
}
