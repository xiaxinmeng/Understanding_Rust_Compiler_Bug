rust
// Increment the strong reference count
let arc = Arc::from_raw(waker as *const W);
arc.incr_strong_count();

// Decrement the strong reference count
let arc = Arc::from_raw(waker as *const W);
arc.decr_strong_count();
