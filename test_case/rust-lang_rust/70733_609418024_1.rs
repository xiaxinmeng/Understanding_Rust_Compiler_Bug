rust
// Increment the strong reference count
Arc::incr_strong_count(waker as *const W);

// Decrement the strong reference count
Arc::decr_strong_count(waker as *const W);
